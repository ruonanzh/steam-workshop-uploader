---
name: steam-workshop
description: Publish an already verified Mod to Steam Workshop, or update its files, cover, gallery, and localized titles and descriptions.
---

# Steam Workshop publishing

Use the native Rust CLI bundled with this skill and the player's existing local Steam session. Steam must be running and signed in, and the account must have the required game license and publishing permissions. Do not request passwords, Steam Guard codes, or API keys. If authentication fails, direct the player to the official Steam client and handle the actual CLI error.

## Host and execution

This is a standard, host-neutral SKILL.md skill. Hosts discover it through its name and description. In Pi projects, install the complete folder at `<game-repository>/.pi/skills/steam-workshop/`, then refresh skill discovery or start a new session. Repository development agents should read the [installation guide](references/installation.md).

Read this file and the relevant references with the host's file-reading tool, then invoke the CLI through its command-execution tool. Pi normally provides `read` and `bash`; use the PowerShell examples when the Windows host provides PowerShell. No skill-specific tool registration is required.

The current game repository comes from the session workspace, not the skill installation directory. Resolve CLI and reference paths relative to this SKILL.md. Resolve `mod-repo.json` and `your_mods/` relative to the current game repository.

## Product preparation tool

When `steam_workshop_prepare` is available, call it with `{}` when preparing an authorized publishing task. It reads this session's persisted Mod binding and the workspace App ID, checks known records, and creates the fixed publishing directories when safe. Use the returned paths, App ID and item ID; do not supply or guess a Mod path. It does not choose/copy content files, write an item ID, contact Steam, or prove submission success. A blocked result must be resolved before publishing. Legacy records are reported for reconciliation, not silently moved or replaced.

`record.state=recorded` means a local item ID is known; `configured` means an App ID record exists without an item ID; `missing` means no canonical record was found. Missing history and remote upload status remain unknown. The product tool checks only its fixed canonical path and three known legacy paths, not every possible old staging directory. Read the relevant publishing records before deciding to create a new item.

Other Pi hosts can use this skill without that product tool. Resolve the current Mod through the host's trusted session context, then prepare the same paths with ordinary file tools. Do not infer the bound Mod from the shell cwd, which can still be the game repository. Game Helper and unbound sessions must use the appropriate Mod-development workflow first.

## Workspace and runtime

- Read `game.steamAppId` from the current repository's root `mod-repo.json`. Report a missing value instead of guessing.
- Work in the current session's Mod folder, `your_mods/<folder name>/`. Locate or ask for the current Mod if it is unclear; do not switch to another Mod implicitly.
- Compilation and in-game verification belong to the existing creation workflow. Use the already verified results for publishing.
- Select the publishing files from the actual Mod content. No game adapter or fixed packing list is required. Keep publishing data under the current Mod's `.preview/steam-workshop/`: selected upload files in `content/`, cover assets in `cover/`, localized text in `text/`, and results in `publish-record.md`. The fixed item record is `.preview/steam-workshop/content/workshop.toml`. This layout does not prescribe which runtime files the Mod needs. Treat this directory as persistent publishing data, not disposable preview cache.
- Read `bundle.json` to select an available native binary. Invoke its absolute path; do not depend on the shell cwd, global PATH, or a developer checkout. Keep the native Steam library beside the executable. If the platform is missing, report the missing package instead of substituting an older CLI.
- macOS uses `bin/macos-universal/workshop` (or `macos-arm64` in an architecture-specific package); Windows uses `bin/windows-x64/workshop.exe`. See the [CLI reference](references/cli.md). Keep player data outside the installed skill folder.

## Default cover

When creating an item and the player has not provided a cover, use `assets/default-cover.png`: the 1024 x 1024 green Mod Agent logo on a solid black background. Do not ask the player to provide or confirm a cover. Omit `--preview`; the CLI applies the same embedded image automatically, including with `--no-prompt`.

When the player provides a cover, pass its absolute path with `--preview`. If an explicit image path is invalid, correct the path instead of silently substituting the default. On updates without a replacement cover, omit `--preview` to preserve the published cover. The cover can be replaced later on the same item with `update --no-content-update --preview <ABSOLUTE_COVER_PATH>`; read the existing item record first.

## Publishing workflow

1. Distinguish creation from updating. Read `.preview/steam-workshop/content/workshop.toml` and `.preview/steam-workshop/publish-record.md` first. For an update, use the user-specified ID or the current Mod's recorded Workshop ID. If only a legacy record exists, reconcile its App ID and item ID before migrating its publishing data into the fixed directory. Preserve originals on conflicts; do not merge different item IDs. A missing record means unknown publishing history, not proof that the Mod was never published. An item ID proves creation, not successful content submission. Query the actual target to check its game and metadata. Public readability and a local ID do not establish ownership; this CLI does not expose an ownership preflight. Rely on Steam's permission result during the requested update and report permission failures without creating a replacement item. Do not create a new item when an update target is unclear.
2. Prepare the title, description, cover, and optional gallery from the Mod and the player's request. Read [image rules](references/images.md) when handling images and [localization rules](references/localization.md) for multiple languages. Do not add unrequested promotion or release claims.
3. Prepare the upload directory and run `files` to inspect the actual file list. Ignore rules can omit compiled artifacts. Make the list match the intended selection; do not upload the entire game repository.
4. Create or update. If creation visibility is unspecified, use private and report that choice. Honor an explicit request for public publishing directly. Preserve visibility on updates unless instructed otherwise. Do not request another confirmation for an already authorized publishing action.
5. Save the returned Workshop ID in `.preview/steam-workshop/content/workshop.toml` and record the actual submission result in `.preview/steam-workshop/publish-record.md`. Reuse the same item for later updates. Keep any runtime Mod identity field consistent with that ID when the Mod format has one. After an uncertain result, query and reconcile before retrying creation or gallery append.
6. Read back each localized title and full description. Check the cover URL and gallery count, filenames, and image loading. Return the Workshop link and the actual verification scope. See [publishing and failure handling](references/publishing.md).

Sharing to the community is a separate action. Continue with an available community-share skill only when the player requests it.
