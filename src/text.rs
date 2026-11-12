use std::{sync::mpsc, time::Duration};

use color_eyre::eyre::{self, ContextCompat, WrapErr};
use serde_json::json;

use crate::{
    cli::{GetTextCommand, SetTextCommand},
    workshop::{self, SteamworksClient},
};

pub fn get_text(args: GetTextCommand) -> eyre::Result<()> {
    let client = SteamworksClient::init(args.app_id)?;
    let (tx, rx) = mpsc::channel();
    client
        .ugc()
        .query_item(args.item_id.into())?
        .include_long_desc(true)
        .language((&args.language).into())
        .fetch(move |result| {
            let _ = tx.send(result.map(|items| items.iter().find_map(|item| item)));
        });
    let item = rx
        .recv_timeout(Duration::from_secs(30))
        .wrap_err("Workshop text query timed out or disconnected")??
        .context("Workshop item was not returned for this account")?;
    println!(
        "{}",
        json!({
            "app_id": args.app_id, "item_id": args.item_id.to_string(),
            "language": args.language.to_string(), "title": item.title,
            "description": item.description, "visibility": format!("{:?}", item.visibility),
        })
    );
    Ok(())
}

pub fn set_text(args: SetTextCommand) -> eyre::Result<()> {
    let description = fs_err::read_to_string(&args.description_file)?;
    workshop::is_valid_title(&args.title)?;
    workshop::is_valid_description(&description)?;
    let client = SteamworksClient::init(args.app_id)?;
    let handle = client
        .ugc()
        .start_item_update(args.app_id.into(), args.item_id.into())
        .language((&args.language).into())
        .title(&args.title)
        .description(&description);
    let (tx, rx) = mpsc::channel();
    let _watch = handle.submit(None, move |result| {
        let _ = tx.send(result);
    });
    let (item_id, needs_agreement) = rx
        .recv_timeout(Duration::from_secs(120))
        .wrap_err("Text submission result is unknown; query the item before retrying")??;
    println!(
        "{}",
        json!({
            "app_id": args.app_id, "item_id": item_id.0.to_string(),
            "language": args.language.to_string(), "status": "updated",
            "needs_workshop_agreement": needs_agreement,
        })
    );
    Ok(())
}
