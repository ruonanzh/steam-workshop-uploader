#!/usr/bin/env python3
"""Verify a portable skill and run local native CLI checks without calling Steam."""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import platform
import subprocess
import struct
import sys
import tempfile


def windows_imports(file, is_library=False):
    data = file.read_bytes()
    if data[:2] != b'MZ' or len(data) < 64:
        raise ValueError('Not a Windows PE file: ' + str(file))
    pe = struct.unpack_from('<I', data, 0x3c)[0]
    if data[pe:pe + 4] != b'PE\0\0' or struct.unpack_from('<H', data, pe + 4)[0] != 0x8664:
        raise ValueError('Windows runtime must be x64: ' + str(file))
    optional = pe + 24
    if struct.unpack_from('<H', data, optional)[0] != 0x20b:
        raise ValueError('Windows runtime must be PE32+: ' + str(file))
    if bool(struct.unpack_from('<H', data, pe + 22)[0] & 0x2000) != is_library:
        raise ValueError('Incorrect EXE/DLL type: ' + str(file))
    count = struct.unpack_from('<H', data, pe + 6)[0]
    sections = optional + struct.unpack_from('<H', data, pe + 20)[0]
    def offset(rva):
        for i in range(count):
            virtual_size, address, raw_size, raw = struct.unpack_from('<IIII', data, sections + i * 40 + 8)
            if address <= rva < address + min(virtual_size, raw_size):
                return raw + rva - address
        raise ValueError('Unmapped Windows PE RVA')
    imports_rva = struct.unpack_from('<I', data, optional + 120)[0]
    imports = []
    cursor = offset(imports_rva)
    for i in range(128):
        descriptor = struct.unpack_from('<IIIII', data, cursor + i * 20)
        if not any(descriptor):
            return imports
        name = offset(descriptor[3])
        end = data.index(0, name, name + 261)
        imports.append(data[name:end].decode('ascii').lower())
    raise ValueError('Unterminated Windows import table')


def verify_and_smoke(skill, target, native=True):
    skill = skill.resolve()
    host = 'windows' if sys.platform == 'win32' else 'macos' if sys.platform == 'darwin' else 'linux'
    if native and not target.startswith(host + '-'):
        raise ValueError('Native smoke checks must run on the matching operating system')
    if native and target == 'windows-x64' and platform.machine().lower() not in ('amd64', 'x86_64'):
        raise ValueError('The Windows package requires an x64 host for this native check')
    manifest = json.loads((skill / 'bundle.json').read_text(encoding='utf-8'))
    for relative, digest in manifest['files'].items():
        parts = PurePosixPath(relative).parts
        if not parts or PurePosixPath(relative).is_absolute() or any(p in ('.', '..') for p in parts) or '\\' in relative or ':' in relative:
            raise ValueError('Unsafe bundle payload path')
        payload = skill / relative
        if payload.is_symlink() or hashlib.sha256(payload.read_bytes()).hexdigest() != digest:
            raise ValueError('Bundle payload hash mismatch: ' + relative)
    runtime = manifest['platforms'][target]
    if runtime['executable'] not in manifest['files'] or runtime['steam_library'] not in manifest['files']:
        raise ValueError('Native executable and Steam library must be hashed payloads')
    cli = skill / runtime['executable']
    windows_dlls = None
    if target == 'windows-x64':
        windows_dlls = windows_imports(cli)
        library_dlls = windows_imports(skill / runtime['steam_library'], is_library=True)
        if 'steam_api64.dll' not in windows_dlls:
            raise ValueError('Windows executable does not link the bundled Steam API')
        if any(name.startswith(('vcruntime', 'msvcp', 'msvcr', 'api-ms-win-crt-')) for name in windows_dlls + library_dlls):
            raise ValueError('Windows runtime requires an extra Visual C++ runtime; build with static CRT')
    if not native:
        return {'platform': target, 'hashesVerified': len(manifest['files']),
                'windowsImports': windows_dlls, 'nativeCliLoaded': False, 'steamSubmission': False}
    with tempfile.TemporaryDirectory(prefix='workshop-native-smoke-') as tmp:
        tmp = Path(tmp)
        content = tmp / 'content with spaces 中文'
        content.mkdir()
        (content / 'readme.txt').write_text('Local validation only.', encoding='utf-8')
        def run(args):
            return subprocess.run([str(cli), *args], cwd=tmp, stdin=subprocess.DEVNULL,
                                  capture_output=True, text=True, encoding='utf-8', errors='replace', timeout=15)
        def require(condition, message):
            if not condition:
                raise RuntimeError(message)
        help_result = run(['--help'])
        require(help_result.returncode == 0, 'Native CLI or Steam DLL could not load: ' + help_result.stderr)
        for command in ['get-text', 'set-text', 'get-previews', 'add-preview']:
            require(command in help_result.stdout, 'Required command is missing: ' + command)
        create_help = run(['create', '--help'])
        require(create_help.returncode == 0 and '1024x1024' in create_help.stdout, 'Default-cover CLI help is missing')
        base = ['--no-prompt', 'create', '--app-id', '3167020', '--content', str(content), '--title', 'x' * 10000]
        default = run(base)
        require(default.returncode != 0 and 'Title can only have a max length' in default.stderr,
                'Create did not stop at local title validation before Steam initialization')
        require('No cover provided; using the built-in 1024x1024' in default.stderr, 'Embedded default cover was not selected')
        explicit = run(base + ['--preview', str(skill / 'assets/default-cover.png')])
        require(explicit.returncode != 0 and 'Title can only have a max length' in explicit.stderr,
                'Explicit cover did not reach local validation')
        require('No cover provided' not in explicit.stderr, 'Explicit cover was replaced by the fallback')
        invalid = run(base + ['--preview', str(tmp / 'missing-cover.png')])
        require(invalid.returncode != 0 and 'No cover provided' not in invalid.stderr, 'Invalid cover path silently used fallback')
        update = run(['--no-prompt', 'update', '--content', str(content), '--no-content-update'])
        require(update.returncode != 0 and 'No cover provided' not in update.stderr, 'Update selected fallback')
        require(not (content / 'workshop.toml').exists(), 'Local smoke unexpectedly created an item record')
    return {'platform': target, 'hashesVerified': len(manifest['files']), 'nativeCliLoaded': True,
            'createDefaultCover': True, 'explicitCoverPreserved': True, 'invalidCoverRejected': True,
            'updateNoFallback': True, 'steamSubmission': False}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('skill', type=Path)
    parser.add_argument('--platform', choices=['windows-x64', 'macos-universal', 'macos-arm64', 'linux-x64'], required=True)
    parser.add_argument('--static', action='store_true', help='Verify hashes and Windows PE imports without execution')
    args = parser.parse_args()
    print(json.dumps(verify_and_smoke(args.skill, args.platform, native=not args.static), indent=2))
