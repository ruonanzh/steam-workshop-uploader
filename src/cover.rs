use std::io::Write;

use color_eyre::eyre;
use tempfile::NamedTempFile;

use crate::{cli::Command, workshop::is_valid_preview_file};

const DEFAULT_COVER: &[u8] = include_bytes!("../skills/steam-workshop/assets/default-cover.png");

/// Only creation gets a fallback. Keep the guard alive until Steam finishes reading.
/// Updates without --preview must preserve the published cover.
pub fn prepare_default_cover(command: &mut Command) -> eyre::Result<Option<NamedTempFile>> {
    let Command::Create(create) = command else {
        return Ok(None);
    };
    if create.workshop_item.preview_path.is_some() {
        return Ok(None);
    }
    let mut image = tempfile::Builder::new()
        .prefix("mod-agent-cover-")
        .suffix(".png")
        .tempfile()?;
    image.write_all(DEFAULT_COVER)?;
    image.flush()?;
    is_valid_preview_file(image.path())?;
    create.workshop_item.preview_path = Some(image.path().to_path_buf());
    eprintln!(
        "[-] No cover provided; using the built-in 1024x1024 Mod Agent logo (black background). The cover can be changed later."
    );
    Ok(Some(image))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cli::Cli, workshop::is_valid_preview_file};
    use clap::Parser;

    #[test]
    fn create_without_cover_gets_a_temporary_1024_png_and_cleans_it_up() {
        let mut cli = Cli::try_parse_from(["workshop", "--no-prompt", "create"]).unwrap();
        let guard = prepare_default_cover(&mut cli.command)
            .unwrap()
            .expect("default cover");
        let Command::Create(create) = &cli.command else {
            panic!("create command")
        };
        let path = create
            .workshop_item
            .preview_path
            .as_ref()
            .expect("preview path");
        assert_eq!(path, guard.path());
        is_valid_preview_file(path).unwrap();
        let png = std::fs::read(path).unwrap();
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
        assert_eq!(u32::from_be_bytes(png[16..20].try_into().unwrap()), 1024);
        assert_eq!(u32::from_be_bytes(png[20..24].try_into().unwrap()), 1024);
        assert!(png.len() < 900_000);
        let path = path.clone();
        drop(guard);
        assert!(!path.exists());
    }

    #[test]
    fn explicit_cover_is_used_unchanged_for_create_and_update() {
        let custom = tempfile::Builder::new().suffix(".png").tempfile().unwrap();
        for subcommand in ["create", "update"] {
            let mut cli = Cli::try_parse_from([
                "workshop",
                subcommand,
                "--preview",
                custom.path().to_str().unwrap(),
            ])
            .unwrap();
            assert!(prepare_default_cover(&mut cli.command).unwrap().is_none());
            let item = match &cli.command {
                Command::Create(c) => &c.workshop_item,
                Command::Update(c) => &c.workshop_item,
                _ => unreachable!(),
            };
            assert_eq!(item.preview_path.as_deref(), Some(custom.path()));
        }
    }

    #[test]
    fn update_without_cover_does_not_replace_the_existing_steam_cover() {
        let mut cli = Cli::try_parse_from(["workshop", "update", "--no-content-update"]).unwrap();
        assert!(prepare_default_cover(&mut cli.command).unwrap().is_none());
        let Command::Update(update) = &cli.command else {
            panic!("update command")
        };
        assert!(update.workshop_item.preview_path.is_none());
    }

    #[test]
    fn an_invalid_explicit_path_is_an_error_instead_of_silent_fallback() {
        assert!(
            Cli::try_parse_from([
                "workshop",
                "create",
                "--preview",
                "/missing-cover-for-test.png"
            ])
            .is_err()
        );
    }
}
