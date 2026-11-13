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

## Workspace and runtime

- Read `game.steamAppId` from the current repository's root `mod-repo.json`. Report a missing value instead of guessing.
- Work in the current session's Mod folder, `your_mods/<folder name>/`. Locate or ask for the current Mod if it is unclear; do not switch to another Mod implicitly.
- Compilation and in-game verification belong to the existing creation workflow. Use the already verified results for publishing.
- Select the publishing files from the actual Mod content. No game adapter or fixed packing list is required. Keep staging files and publishing records inside the current Mod, for example under `.workshop/`.
- Read `bundle.json` to select an available native binary. Invoke its absolute path; do not depend on the shell cwd, global PATH, or a developer checkout. Keep the native Steam library beside the executable. If the platform is missing, report the missing package instead of substituting an older CLI.
- macOS uses `bin/macos-universal/workshop` (or `macos-arm64` in an architecture-specific package); Windows uses `bin/windows-x64/workshop.exe`. See the [CLI reference](references/cli.md). Keep player data outside the installed skill folder.

## Publishing workflow

1. Distinguish creation from updating. For an update, use the user-specified ID or the current Mod's recorded Workshop ID. Query the actual target to check its game and the account's access; a local ID alone does not establish ownership. Do not create a new item when an update target is unclear.
2. Prepare the title, description, cover, and optional gallery from the Mod and the player's request. Read [image rules](references/images.md) when handling images and [localization rules](references/localization.md) for multiple languages. Do not add unrequested promotion or release claims.
3. Prepare the upload directory and run `files` to inspect the actual file list. Ignore rules can omit compiled artifacts. Make the list match the intended selection; do not upload the entire game repository.
4. Create or update. If creation visibility is unspecified, use private and report that choice. Honor an explicit request for public publishing directly. Preserve visibility on updates unless instructed otherwise. Do not request another confirmation for an already authorized publishing action.
5. Save the returned Workshop ID and `workshop.toml` inside the current Mod. Reuse the same item for later updates. Keep any runtime Mod identity field consistent with that ID when the Mod format has one. After an uncertain result, query and reconcile before retrying creation or gallery append.
6. Read back each localized title and full description. Check the cover URL and gallery count, filenames, and image loading. Return the Workshop link and the actual verification scope. See [publishing and failure handling](references/publishing.md).

Sharing to the community is a separate action. Continue with an available community-share skill only when the player requests it.
