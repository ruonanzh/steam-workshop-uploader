use std::{
    ffi::{CStr, CString},
    mem::{self, MaybeUninit},
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use color_eyre::eyre::{self, ContextCompat, bail, ensure};
use serde::Serialize;
use serde_json::json;
use steamworks_sys as sys;

use crate::{
    cli::{AddPreviewCommand, GetPreviewsCommand},
    workshop,
};

// steamworks-rs does not expose gallery mutation/result accessors. Keep this
// short-lived native session separate from its manual callback dispatcher.
struct Session {
    ugc: *mut sys::ISteamUGC,
    utils: *mut sys::ISteamUtils,
    user: *mut sys::ISteamUser,
    app_id: u32,
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { sys::SteamAPI_Shutdown() };
    }
}

struct Query {
    ugc: *mut sys::ISteamUGC,
    handle: u64,
}
impl Drop for Query {
    fn drop(&mut self) {
        unsafe { sys::SteamAPI_ISteamUGC_ReleaseQueryUGCRequest(self.ugc, self.handle) };
    }
}

#[derive(Serialize)]
struct Preview {
    index: u32,
    kind: u32,
    filename: String,
    url: String,
}

#[derive(Serialize)]
struct Gallery {
    app_id: u32,
    item_id: String,
    primary_preview_url: String,
    visibility: u32,
    previews: Vec<Preview>,
    #[serde(skip_serializing)]
    owner: u64,
}

impl Session {
    fn new(app_id: u32) -> eyre::Result<Self> {
        let cwd = std::env::current_dir()?;
        let temp = tempfile::tempdir()?;
        fs_err::write(temp.path().join("steam_appid.txt"), app_id.to_string())?;
        std::env::set_current_dir(temp.path())?;
        let mut error: sys::SteamErrMsg = [0; 1024];
        let result = unsafe { sys::SteamAPI_InitFlat(&mut error) };
        let restored = std::env::set_current_dir(cwd);
        if result != sys::ESteamAPIInitResult::k_ESteamAPIInitResult_OK {
            bail!(
                "Steam initialization failed: {}",
                unsafe { CStr::from_ptr(error.as_ptr()) }.to_string_lossy()
            );
        }
        let session = Self {
            ugc: unsafe { sys::SteamAPI_SteamUGC_v021() },
            utils: unsafe { sys::SteamAPI_SteamUtils_v010() },
            user: unsafe { sys::SteamAPI_SteamUser_v023() },
            app_id,
        };
        restored?;
        ensure!(
            !session.ugc.is_null() && !session.utils.is_null() && !session.user.is_null(),
            "Steam interface unavailable"
        );
        ensure!(
            unsafe { sys::SteamAPI_ISteamUser_BLoggedOn(session.user) },
            "Steam user is not logged on"
        );
        ensure!(
            unsafe { sys::SteamAPI_ISteamUtils_GetAppID(session.utils) } == app_id,
            "Wrong Steam application context"
        );
        Ok(session)
    }

    fn wait<T>(&self, call: u64, callback: i32, timeout: Duration) -> eyre::Result<T> {
        ensure!(call != 0, "Steam returned an invalid API call");
        let deadline = Instant::now() + timeout;
        loop {
            unsafe { sys::SteamAPI_RunCallbacks() };
            let mut failed = false;
            if unsafe {
                sys::SteamAPI_ISteamUtils_IsAPICallCompleted(self.utils, call, &mut failed)
            } {
                ensure!(
                    !failed,
                    "Steam API call failed; query remote state before retrying"
                );
                let mut result = MaybeUninit::<T>::uninit();
                let ok = unsafe {
                    sys::SteamAPI_ISteamUtils_GetAPICallResult(
                        self.utils,
                        call,
                        result.as_mut_ptr().cast(),
                        mem::size_of::<T>() as i32,
                        callback,
                        &mut failed,
                    )
                };
                ensure!(
                    ok && !failed,
                    "Steam result unavailable; query remote state before retrying"
                );
                // The matching callback ID and exact generated ABI size were accepted.
                return Ok(unsafe { result.assume_init() });
            }
            ensure!(
                Instant::now() < deadline,
                "Steam result is unknown after timeout; list previews before retrying"
            );
            thread::sleep(Duration::from_millis(50));
        }
    }

    fn gallery(&self, item_id: u64) -> eyre::Result<Gallery> {
        let mut id = item_id;
        let handle =
            unsafe { sys::SteamAPI_ISteamUGC_CreateQueryUGCDetailsRequest(self.ugc, &mut id, 1) };
        ensure!(handle != u64::MAX, "Invalid preview query handle");
        let query = Query {
            ugc: self.ugc,
            handle,
        };
        ensure!(
            unsafe { sys::SteamAPI_ISteamUGC_SetReturnAdditionalPreviews(self.ugc, handle, true) },
            "Could not request gallery previews"
        );
        ensure!(
            unsafe { sys::SteamAPI_ISteamUGC_SetAllowCachedResponse(self.ugc, handle, 0) },
            "Could not disable cached response"
        );
        let call = unsafe { sys::SteamAPI_ISteamUGC_SendQueryUGCRequest(self.ugc, handle) };
        let result: sys::SteamUGCQueryCompleted_t =
            self.wait(call, 3401, Duration::from_secs(30))?;
        ensure!(
            result.m_eResult == sys::EResult::k_EResultOK && result.m_unNumResultsReturned == 1,
            "Preview query failed: {:?}",
            result.m_eResult
        );
        let mut details = MaybeUninit::<sys::SteamUGCDetails_t>::uninit();
        ensure!(
            unsafe {
                sys::SteamAPI_ISteamUGC_GetQueryUGCResult(self.ugc, handle, 0, details.as_mut_ptr())
            },
            "Item details unavailable"
        );
        let details = unsafe { details.assume_init() };
        ensure!(
            details.m_eResult == sys::EResult::k_EResultOK && details.m_nPublishedFileId == item_id,
            "Wrong item returned"
        );
        ensure!(
            details.m_nConsumerAppID == self.app_id,
            "Item belongs to another game"
        );
        let mut primary = [0; 4096];
        unsafe {
            sys::SteamAPI_ISteamUGC_GetQueryUGCPreviewURL(
                self.ugc,
                handle,
                0,
                primary.as_mut_ptr(),
                primary.len() as u32,
            )
        };
        let count = unsafe {
            sys::SteamAPI_ISteamUGC_GetQueryUGCNumAdditionalPreviews(self.ugc, handle, 0)
        };
        let mut previews = Vec::new();
        for index in 0..count {
            let mut url = [0; 4096];
            let mut filename = [0; 1024];
            let mut kind = sys::EItemPreviewType::k_EItemPreviewType_Image;
            ensure!(
                unsafe {
                    sys::SteamAPI_ISteamUGC_GetQueryUGCAdditionalPreview(
                        self.ugc,
                        handle,
                        0,
                        index,
                        url.as_mut_ptr(),
                        url.len() as u32,
                        filename.as_mut_ptr(),
                        filename.len() as u32,
                        &mut kind,
                    )
                },
                "Gallery preview unavailable"
            );
            previews.push(Preview {
                index,
                kind: kind as u32,
                filename: unsafe { CStr::from_ptr(filename.as_ptr()) }
                    .to_string_lossy()
                    .into_owned(),
                url: unsafe { CStr::from_ptr(url.as_ptr()) }
                    .to_string_lossy()
                    .into_owned(),
            });
        }
        drop(query);
        Ok(Gallery {
            app_id: self.app_id,
            item_id: item_id.to_string(),
            primary_preview_url: unsafe { CStr::from_ptr(primary.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
            visibility: details.m_eVisibility as u32,
            previews,
            owner: details.m_ulSteamIDOwner,
        })
    }
}

pub fn get_previews(args: GetPreviewsCommand) -> eyre::Result<()> {
    let session = Session::new(args.app_id)?;
    println!(
        "{}",
        serde_json::to_string(&session.gallery(args.item_id)?)?
    );
    Ok(())
}

pub fn add_preview(args: AddPreviewCommand) -> eyre::Result<()> {
    let file: PathBuf = args.file.canonicalize()?;
    workshop::is_valid_preview_file(&file)?;
    ensure!(
        fs_err::metadata(&file)?.len() < workshop::MAX_PREVIEW_SIZE_BYTES,
        "Gallery image must be smaller than 1 MiB"
    );
    let filename = file
        .file_name()
        .and_then(|s| s.to_str())
        .context("Image filename must be UTF-8")?;
    let native_path = CString::new(file.to_str().context("Image path must be UTF-8")?)?;
    let session = Session::new(args.app_id)?;
    let before = session.gallery(args.item_id)?;
    ensure!(
        before.owner == unsafe { sys::SteamAPI_ISteamUser_GetSteamID(session.user) },
        "Only the item's owner may append images"
    );
    ensure!(
        !before.previews.iter().any(|p| p.filename == filename),
        "Preview filename already exists; refusing a duplicate append"
    );
    let handle =
        unsafe { sys::SteamAPI_ISteamUGC_StartItemUpdate(session.ugc, args.app_id, args.item_id) };
    ensure!(handle != u64::MAX, "Invalid update handle");
    ensure!(
        unsafe {
            sys::SteamAPI_ISteamUGC_AddItemPreviewFile(
                session.ugc,
                handle,
                native_path.as_ptr(),
                sys::EItemPreviewType::k_EItemPreviewType_Image,
            )
        },
        "Gallery image was rejected"
    );
    let call =
        unsafe { sys::SteamAPI_ISteamUGC_SubmitItemUpdate(session.ugc, handle, std::ptr::null()) };
    let result: sys::SubmitItemUpdateResult_t =
        session.wait(call, 3404, Duration::from_secs(120))?;
    ensure!(
        result.m_eResult == sys::EResult::k_EResultOK,
        "Gallery upload failed: {:?}",
        result.m_eResult
    );
    let after = session.gallery(args.item_id)?;
    let verified = after.previews.len() == before.previews.len() + 1
        && after
            .previews
            .iter()
            .filter(|p| p.filename == filename)
            .count()
            == 1
        && after.primary_preview_url == before.primary_preview_url
        && after.visibility == before.visibility;
    println!(
        "{}",
        json!({"app_id": args.app_id, "item_id": args.item_id.to_string(),
        "status": "uploaded", "verified": verified, "needs_workshop_agreement": result.m_bUserNeedsToAcceptWorkshopLegalAgreement,
        "before_count": before.previews.len(), "gallery": after })
    );
    ensure!(
        verified,
        "Upload completed but readback differs; do not blindly append again"
    );
    Ok(())
}
