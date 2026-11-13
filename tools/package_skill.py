#!/usr/bin/env python3
"""Build a portable skill directory and ZIP from locally built native artifacts."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import zipfile

ROOT = Path(__file__).resolve().parents[1]
PLATFORMS = {
    "macos-universal": ("workshop", "libsteam_api.dylib"),
    "macos-arm64": ("workshop", "libsteam_api.dylib"),
    "windows-x64": ("workshop.exe", "steam_api64.dll"),
    "linux-x64": ("workshop", "libsteam_api.so"),
}


def package(platform, binary, library, output):
    binary, library, output = binary.resolve(), library.resolve(), output.resolve()
    expected_binary, expected_library = PLATFORMS[platform]
    if not binary.is_file() or not library.is_file():
        raise ValueError("Both native executable and Steam library are required")
    destination = output / "steam-workshop"
    if destination.exists():
        raise ValueError(f"Output already exists: {destination}; use a new output directory")
    shutil.copytree(ROOT / "skills/steam-workshop", destination)
    runtime = destination / "bin" / platform
    runtime.mkdir(parents=True)
    shutil.copy2(binary, runtime / expected_binary)
    shutil.copy2(library, runtime / expected_library)
    if platform != "windows-x64":
        (runtime / expected_binary).chmod(0o755)
    shutil.copy2(destination / "config.toml", runtime / "config.toml")
    licenses = destination / "LICENSES"
    licenses.mkdir()
    shutil.copy2(ROOT / "LICENSE", licenses / "workshop-GPL-3.0.txt")
    (licenses / "Steamworks-NOTICE.txt").write_text(
        "The bundled Steam API runtime is proprietary software by Valve Corporation.\n"
        "It is an SDK redistributable and is governed by Valve's Steamworks SDK Access Agreement,\n"
        "not by the GPL license of this uploader or the license of its Rust bindings.\n"
        "https://partner.steamgames.com/documentation/sdk_access_agreement/\n", encoding="utf-8")
    # Include corresponding uploader source, with no build cache, player data,
    # local Git metadata, or unrelated research/test artifacts.
    source = destination / "source"
    source.mkdir()
    for name in ["src", "plugins", ".cargo", "tools", "skills"]:
        shutil.copytree(ROOT / name, source / name,
                        ignore=shutil.ignore_patterns("target", "__pycache__", "*.pyc", "bin"))
    for name in ["Cargo.toml", "Cargo.lock", "LICENSE", "README.md"]:
        shutil.copy2(ROOT / name, source / name)
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    manifest = {
        "schema_version": 1,
        "name": "steam-workshop",
        "version": (destination / "VERSION").read_text().strip(),
        "source_revision": revision,
        "upstream": "https://github.com/nozwock/steam-workshop-uploader",
        "platforms": {platform: {"executable": f"bin/{platform}/{expected_binary}",
                                  "steam_library": f"bin/{platform}/{expected_library}"}},
        "files": {},
    }
    for path in sorted(destination.rglob("*")):
        if path.is_file():
            manifest["files"][path.relative_to(destination).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    (destination / "bundle.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    archive = output / f"steam-workshop-{manifest['version']}-{platform}.zip"
    with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as bundle:
        for path in sorted(destination.rglob("*")):
            if path.is_file():
                bundle.write(path, path.relative_to(output))
    print(json.dumps({"directory": str(destination), "archive": str(archive),
                      "platform": platform, "files": len(manifest["files"])}, indent=2))
    return destination, archive


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--platform", choices=PLATFORMS, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--steam-library", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    package(args.platform, args.binary, args.steam_library, args.output)
