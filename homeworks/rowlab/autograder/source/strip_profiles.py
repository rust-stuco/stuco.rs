"""Remove all `[profile]` settings from a student's `Cargo.toml`.

This makes every submission build and benchmark with Cargo's default profiles, so the leaderboard
compares code rather than compiler settings (for example, `lto = "fat"` alone speeds up the
measurement generator by about 20%).
"""

import sys

import tomli
import tomli_w


def main(path):
    with open(path, "rb") as f:
        try:
            manifest = tomli.load(f)
        except tomli.TOMLDecodeError:
            # Leave an invalid manifest alone so `cargo build` reports the error to the student.
            return

    if manifest.pop("profile", None) is None:
        return

    with open(path, "wb") as f:
        tomli_w.dump(manifest, f)


if __name__ == "__main__":
    main(sys.argv[1])
