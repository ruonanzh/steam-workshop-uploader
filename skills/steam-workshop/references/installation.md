# Install the skill in a Mod repository

Audience: an Agent maintaining a game Mod repository for Pi Agent or pi-desktop. Install the complete portable skill package so the player's Agent can publish already verified Mods. Compilation, game verification, and Mod-specific file selection remain in the existing creation workflow.

## Select the source and target

Use a complete release package containing SKILL.md, references/, bundle.json, bin/, licenses, and corresponding source. The development template at `nozwock-workshop/skills/steam-workshop/` alone has no executable. Use the exported `steam-workshop/` directory or extract a release ZIP first.

Resolve `<MOD_REPO_ROOT>` to the intended game repository's development checkout. Read its repository instructions. For pi-desktop-managed player clones, deliver maintained repository changes through the normal release/update process rather than editing the managed environment or running out-of-band Git operations there.

Read bundle.json and confirm it contains a native binary for the player's operating system and architecture. The currently supplied Mac Universal package contains ARM64 and x86_64 code in one executable; only Apple Silicon has native runtime validation. It is not a Windows package. Windows build automation exists, but a Windows artifact must actually be produced and checked before shipping to Windows players. Do not replace a missing platform binary with an older upstream release that lacks localization/gallery commands.

## Copy the complete directory

Destination:

```text
<MOD_REPO_ROOT>/.pi/skills/steam-workshop/
  SKILL.md
  references/
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

The player's current Mod is `your_mods/<folder name>/`. No per-game packing manifest is required by this skill. Preserve the current session's Mod binding. Publishing metadata and temporary files belong inside that Mod, not the installed skill. No passwords, API keys, Steam configuration, player Mods, test screenshots, or publishing records belong in the copied package.

Do not add special IPC, MCP tools, model configuration, or host-specific agent metadata. The host's ordinary file-reading and command-execution tools are sufficient. Another Agent runtime can use the same standard SKILL.md directory in its own discovery location.

## Validate installation

1. Verify files against bundle.json and confirm all relative Markdown reference links resolve inside the skill.
2. Run the installed binary's --help from the game repository, using its absolute path. Confirm get-text, set-text, get-previews, and add-preview are available. This does not require publishing an item.
3. Refresh the host's skill discovery if supported; otherwise start a new session. Pi should discover exactly one steam-workshop skill with the installed SKILL.md path and no diagnostics. Do not invent a reload tool if the host does not expose one.
4. Ask the Agent to locate the current repository App ID, current Mod directory, and installed CLI. Discovery and a read-only check are enough for installation validation; they do not authorize a new Workshop upload.
5. Report the installed path, bundle version, available platform, discovery result, and checks actually performed. Follow the target repository's own commit and release process to distribute the skill to players.

The delivered package was validated with the Pi loader installed in pi-desktop: a temporary project installation under .pi/skills/ was discovered once, with no diagnostics, and its relocated CLI ran from a different working directory.
