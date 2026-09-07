"""Check the contents of downloadable homework archives."""

import subprocess
import tempfile
import unittest
from pathlib import Path
from zipfile import ZipFile

from build import create_archive


class ArchiveTests(unittest.TestCase):
    def test_archive_contains_current_visible_sources(self):
        with tempfile.TemporaryDirectory() as directory:
            repository = Path(directory)
            source = repository / "assignment"
            subprocess.run(["git", "init", "--quiet", repository], check=True)
            subprocess.run(
                ["git", "-C", repository, "config", "core.fsmonitor", "false"], check=True
            )

            tracked = {
                "src/lib.rs": "pub fn original() {}\n",
                "src/nested/mod.rs": "pub const VALUE: u8 = 1;\n",
                "src/removed.rs": "deleted after staging\n",
                "Cargo.lock": "tracked before the ignore rule\n",
                ".hidden": "tracked hidden file\n",
            }
            for name, contents in tracked.items():
                file = source / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_text(contents, encoding="utf-8")
            subprocess.run(
                ["git", "-C", repository, "add", "assignment"], check=True
            )
            (source / "src/removed.rs").unlink()

            (repository / ".gitignore").write_text(
                "Cargo.lock\ntarget/\n*.zip\n", encoding="utf-8"
            )
            changes = {
                "src/lib.rs": "pub fn edited() {}\n",
                "src/new.rs": "pub fn untracked() {}\n",
                "target/generated.rs": "generated file\n",
                "handin.zip": "generated archive\n",
                "src/.scratch/notes.rs": "hidden directory\n",
            }
            for name, contents in changes.items():
                file = source / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_text(contents, encoding="utf-8")

            output = repository / "assignment.zip"
            create_archive(source, output)

            with ZipFile(output) as archive:
                actual = {
                    name: archive.read(name).decode("utf-8")
                    for name in archive.namelist()
                }
            self.assertEqual(
                actual,
                {
                    "assignment/src/lib.rs": "pub fn edited() {}\n",
                    "assignment/src/nested/mod.rs": "pub const VALUE: u8 = 1;\n",
                    "assignment/src/new.rs": "pub fn untracked() {}\n",
                },
            )


if __name__ == "__main__":
    unittest.main()
