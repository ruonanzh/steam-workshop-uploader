# CLI reference

`<CLI>` means the absolute path to the executable for the current platform inside the installed skill directory. `<CONTENT>` is `<CURRENT_MOD>/.preview/steam-workshop/content/`, containing the selected publishing files. It is not the whole Mod or `.preview/` directory. Quote paths containing spaces or non-ASCII characters according to the host shell.

Use the host's existing command-execution tool. Pi can invoke the native executable through bash; another Agent can use its equivalent tool. `<CLI>` is not a registered tool name. There is no assumed SKILL_DIR environment variable: derive the real location from the loaded SKILL.md path.

macOS/Bash: `"<ABSOLUTE_SKILL_DIR>/bin/macos-universal/workshop" --help`

Windows/PowerShell: `& "<ABSOLUTE_SKILL_DIR>/bin/windows-x64/workshop.exe" --help`

In pi-desktop, call `steam_workshop_prepare` with `{}` first and use its returned `contentDirectory`, record path, App ID and optional item ID. It performs only local preparation; the following CLI commands still perform actual publishing. Other hosts use the manual path-resolution instructions in SKILL.md. Before mutations, establish the App ID, item, and player's intent. Use `--no-prompt` to disable terminal interaction. Runtime use needs no Rust, Node, or Python installation; those are development tools only.

```text
<CLI> files <CONTENT>
<CLI> --no-prompt create --app-id <APP_ID> --content <CONTENT> --language english --title "Title" --description-file <ABSOLUTE_PATH/en.txt> --visibility private
<CLI> --no-prompt update --content <CONTENT> --language english --description-file <ABSOLUTE_PATH/en.txt>
<CLI> --no-prompt update --content <CONTENT> --language english --no-content-update --preview <ABSOLUTE_COVER_PATH>
<CLI> get-text --app-id <APP_ID> --item-id <ITEM_ID> --language english
<CLI> set-text --app-id <APP_ID> --item-id <ITEM_ID> --language japanese --title "Localized title" --description-file <ABSOLUTE_PATH/ja.txt>
<CLI> get-previews --app-id <APP_ID> --item-id <ITEM_ID>
<CLI> add-preview --app-id <APP_ID> --item-id <ITEM_ID> --file <ABSOLUTE_SCREENSHOT_PATH>
```

`create` without `--preview` uses the embedded 1024 x 1024 Mod Agent logo on pure black. This default works without terminal interaction and does not depend on the working directory or writable skill files. To supply a custom cover, add `--preview <ABSOLUTE_COVER_PATH>`. `update` without `--preview` preserves the current cover. The cover-only update example above replaces the cover on the same item without uploading content again.

create saves its returned ID in `<CONTENT>/workshop.toml`. update reads app_id and item_id from that file. To establish a local record for an existing item:

```text
<CLI> --no-prompt init --app-id <APP_ID> --item-id <ITEM_ID> --content <CONTENT>
```

init is local and does not prove item ownership. Query the intended target with get-previews/get-text first to check metadata; these queries do not prove update permission. workshop.toml is automatically excluded from uploaded content. The stable record path is `<CURRENT_MOD>/.preview/steam-workshop/content/workshop.toml`. Read it before creating or updating; creation writes the ID before content submission, so record success separately in `../publish-record.md`. Keep publishing records out of the installed skill folder.

get-text, set-text, get-previews, and add-preview emit JSON on stdout; native Steam logs may appear on stderr. Generic create/update commands still use human-readable output and file records. Do not assume every command returns JSON. Check process results, remote readback, and records.

A successful add-preview reports the new gallery and verified. After a timeout or verified=false, query get-previews before retrying; Steam may have completed the upload even if the local wait failed.
