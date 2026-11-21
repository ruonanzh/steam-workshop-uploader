# Install the skill in a Mod repository

Audience: an Agent maintaining a game Mod repository for Pi Agent or pi-desktop. Install the complete portable skill package so the player's Agent can publish already verified Mods. Compilation, game verification, and Mod-specific file selection remain in the existing creation workflow.

## Select the source and target

Use a complete release package containing SKILL.md, references/, assets/, bundle.json, bin/, licenses, and corresponding source. The development template at `nozwock-workshop/skills/steam-workshop/` alone has no executable. Use the exported `steam-workshop/` directory or extract a release ZIP first.

Resolve `<MOD_REPO_ROOT>` to the intended game repository's development checkout. Read its repository instructions. For pi-desktop-managed player clones, deliver maintained repository changes through the normal release/update process rather than editing the managed environment or running out-of-band Git operations there.

Read bundle.json and select the native binary for the player's operating system and architecture. The supplied multi-platform package contains Mac Universal (ARM64 and x86_64) and Windows x64 runtimes. Windows uses `bin/windows-x64/workshop.exe` with `steam_api64.dll` beside it. The Windows executable links the C runtime statically; players do not need Rust, Python, SteamCMD, or a separate Visual C++ runtime for this uploader. Steam must be running and signed in, with the required game license and publishing permissions. No game launch or separate password entry is required by this workflow.

Windows x64 artifacts are cross-built and checked as native PE files. The Windows CI gates run source tests and the packaged CLI's local checks without contacting Steam. A configured gate is not evidence that its current run passed. Native Windows Steam upload/update and signed desktop release acceptance require separate Windows-host evidence; do not claim those from macOS checks. Only Apple Silicon has local native runtime validation. Do not replace a missing platform binary with an older upstream release that lacks localization/gallery commands.

## Copy the complete directory

Destination:

```text
<MOD_REPO_ROOT>/.pi/skills/steam-workshop/
  SKILL.md
  references/
  assets/default-cover.png
  bundle.json
  bin/<platform>/workshop[.exe]
  bin/<platform>/<Steam runtime library>
  VERSION
  LICENSES/
  source/
```

If a destination already exists, compare versions and bundle file hashes before upgrading. Preserve local modifications and resolve conflicts rather than merging two runtime versions. Keep the executable and its matching Steam library together.

For a new installation using a POSIX shell, substitute real absolute paths:

```sh
skill_source="/absolute/path/to/extracted/steam-workshop"
mod_repo="/absolute/path/to/game-repository"
destination="$mod_repo/.pi/skills/steam-workshop"
if [ -e "$destination" ]; then
  echo "Destination exists; inspect the existing installation first." >&2
  exit 1
fi
mkdir -p "$mod_repo/.pi/skills"
cp -R "$skill_source" "$destination"
"$destination/bin/macos-universal/workshop" --help
```

PowerShell, after selecting an actual Windows package:

```powershell
$skillSource = 'C:\absolute\extracted\steam-workshop'
$modRepo = 'C:\absolute\game-repository'
$destination = Join-Path $modRepo '.pi\skills\steam-workshop'
if (Test-Path -LiteralPath $destination) {
    throw 'Destination exists; inspect the existing installation first.'
}
New-Item -ItemType Directory -Force -Path (Split-Path $destination) | Out-Null
Copy-Item -LiteralPath $skillSource -Destination $destination -Recurse
& (Join-Path $destination 'bin\windows-x64\workshop.exe') --help
```

## Check repository context

The repository needs only its existing root mod-repo.json with game.steamAppId. Confirm that this is the real game ID; do not invent new adapter fields or use the skill's directory as the game root.

The player's current Mod is `your_mods/<folder name>/`. No per-game packing manifest is required by this skill. Preserve the current session's Mod binding. Publishing data belongs under that Mod's `.preview/steam-workshop/`, with the stable item record at `content/workshop.toml`, not in the installed skill. Keep these records when cleaning preview images. Confirm the game's installer excludes `.preview/`; Duckov already does, but other Mod repositories must be checked rather than assumed. No passwords, API keys, Steam configuration, player Mods, test screenshots, or publishing records belong in the copied package.

No repository-specific IPC, MCP tool, model configuration, or host-specific agent metadata is needed. pi-desktop supplies the optional global `steam_workshop_prepare` tool; copying this skill into another host does not register that tool. The host's ordinary file-reading and command-execution tools are sufficient. Another Agent runtime can use the same standard SKILL.md directory in its own discovery location.

## Validate installation

1. Verify files against bundle.json and confirm all relative Markdown reference links resolve inside the skill.
2. Run the installed binary's --help from the game repository, using its absolute path. Confirm get-text, set-text, get-previews, and add-preview are available. This does not require publishing an item. Developers can run `python source/tools/smoke_skill.py <ABSOLUTE_SKILL_DIR> --platform windows-x64` on Windows x64 (or `--platform macos-universal` on macOS) to verify hashes and local CLI behavior without a Steam upload. Add `--static` to inspect Windows PE imports from a different host; this does not execute the EXE or validate a Steam session.
3. Refresh the host's skill discovery if supported; otherwise start a new session. Pi should discover exactly one steam-workshop skill with the installed SKILL.md path and no diagnostics. Do not invent a reload tool if the host does not expose one.
4. Ask the Agent to locate the current repository App ID, current Mod directory, and installed CLI. Discovery and a read-only check are enough for installation validation; they do not authorize a new Workshop upload.
5. Report the installed path, bundle version, available platform, discovery result, and checks actually performed. Follow the target repository's own commit and release process to distribute the skill to players.

The delivered package was validated with the Pi loader installed in pi-desktop: a temporary project installation under .pi/skills/ was discovered once, with no diagnostics, and its relocated CLI ran from a different working directory.
