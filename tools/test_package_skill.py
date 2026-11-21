import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
import zipfile

spec = importlib.util.spec_from_file_location('package_skill', Path(__file__).with_name('package_skill.py'))
packager = importlib.util.module_from_spec(spec)
spec.loader.exec_module(packager)


class PortableSkillPackaging(unittest.TestCase):
    def test_combined_package_contains_both_runtimes_matching_source_and_default_cover(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            mac = tmp / 'workshop'
            mac.write_bytes(b'mac test executable')
            maclib = tmp / 'libsteam_api.dylib'
            maclib.write_bytes(b'mac test library')
            exe = tmp / 'workshop.exe'
            exe.write_bytes(b'windows test executable')
            dll = tmp / 'steam_api64.dll'
            dll.write_bytes(b'windows test library')
            folder, archive = packager.package('macos-universal', mac, maclib, tmp / 'out',
                additional_runtimes=[('windows-x64', exe, dll)])
            m = json.loads((folder / 'bundle.json').read_text())
            self.assertEqual(set(m['platforms']), {'macos-universal', 'windows-x64'})
            self.assertEqual(m['platforms']['windows-x64']['executable'], 'bin/windows-x64/workshop.exe')
            self.assertEqual((folder / 'bin/windows-x64/steam_api64.dll').read_bytes(), dll.read_bytes())
            self.assertEqual((folder / 'bin/macos-universal/workshop').read_bytes(), mac.read_bytes())
            self.assertEqual((folder / 'assets/default-cover.png').read_bytes(),
                             (folder / 'source/skills/steam-workshop/assets/default-cover.png').read_bytes())
            self.assertFalse(any(p.name == '.DS_Store' for p in folder.rglob('*')))
            self.assertIn('multi-platform', archive.name)
            with zipfile.ZipFile(archive) as z:
                for relative, digest in m['files'].items():
                    self.assertEqual(hashlib.sha256(z.read('steam-workshop/' + relative)).hexdigest(), digest)

    def test_missing_extra_runtime_and_duplicate_platform_fail_before_output(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            binary, library = tmp / 'workshop', tmp / 'library'
            binary.write_bytes(b'bin')
            library.write_bytes(b'lib')
            for extra in [[('windows-x64', tmp / 'missing.exe', library)],
                          [('macos-universal', binary, library)]]:
                with self.subTest(extra=extra):
                    with self.assertRaises(ValueError):
                        packager.package('macos-universal', binary, library, tmp / 'out', additional_runtimes=extra)
                    self.assertFalse((tmp / 'out/steam-workshop').exists())


if __name__ == '__main__':
    unittest.main()
