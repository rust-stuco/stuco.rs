"""Build the website and all published course assets. Requires Python 3.11+.

Run from a Git checkout after installing the tools listed in CONTRIBUTING.md.
Generated assets go into the finished Dioxus bundle, outside its asset processing.
"""

import os
from pathlib import Path
import shutil
import subprocess
import zipfile


ROOT = Path(__file__).resolve().parent.parent
OUTPUT = ROOT / "target/dx/stuco-rs/release/web/public"
SLIDEV = ROOT / "slidev"

LECTURES = [
    "01_introduction",
    "02_ownership_p1",
    "03_structs_enums",
    "04_collections_generics",
    "05_errors_traits",
    "06_modules_testing",
    "07_ecosystem",
    "08_closures_iterators",
    "09_ownership_p2",
    "10_lifetimes",
    "11_smart_pointers",
    "12_unsafe",
    "13_parallelism",
    "14_concurrency",
]

HOMEWORKS = [
    "homeworks/week1/primerlab",
    "homeworks/week2/getownedlab",
    "homeworks/week3/cardlab",
    "homeworks/week4/multilab",
    "homeworks/week5/pokerlab",
    "homeworks/week5-ec/summarylab",
    "homeworks/week6/greplab",
    "homeworks/week8/iterlab",
    "homeworks/week10/splitlab",
    "homeworks/week11/filterlab",
    "homeworks/week13/rowlab",
]


def require_file(path: Path, prefix: bytes = b"") -> None:
    """Require a nonempty output file with the optional byte prefix."""
    with path.open("rb") as file:
        start = file.read(max(1, len(prefix)))

    if not start or not start.startswith(prefix):
        raise ValueError(f"Missing or invalid build output: {path}")


def create_archive(source: Path, output: Path) -> None:
    """Archive working-tree files under source.name, excluding hidden and Git-ignored files."""
    files = subprocess.check_output(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "."],
        cwd=source,
    )

    # ls-files includes tracked files even if they now match an ignore rule.
    ignored = subprocess.run(
        ["git", "check-ignore", "--no-index", "-z", "--stdin"],
        cwd=source, input=files, stdout=subprocess.PIPE,
    )
    # git check-ignore returns 1 when no paths match.
    if ignored.returncode != 1:
        ignored.check_returncode()

    included = set(files.split(b"\0")) - set(ignored.stdout.split(b"\0")) - {b""}
    with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name in sorted(included):
            path = Path(os.fsdecode(name))
            if any(part.startswith(".") for part in path.parts):
                continue

            if (source / path).is_file():
                archive.write(source / path, (Path(source.name) / path).as_posix())


def main() -> None:
    subprocess.run(["dx", "build", "--release"], cwd=ROOT, check=True)
    require_file(OUTPUT / "index.html")
    require_file(OUTPUT / "_redirects")

    # Dioxus processes JavaScript in public_dir. Assemble decks only after it finishes so their
    # module graph survives unchanged. Clear generated destinations to discard removed assets.
    for name in ["lectures", "hw"]:
        destination = OUTPUT / name
        if destination.exists():
            shutil.rmtree(destination)
        destination.mkdir()

    subprocess.run(
        ["typst", "compile", ROOT / "src/syllabus.typ", OUTPUT / "syllabus.pdf"],
        cwd=ROOT, check=True,
    )
    require_file(OUTPUT / "syllabus.pdf", b"%PDF-")

    for homework in HOMEWORKS:
        source = ROOT / homework
        slug = source.name
        destination = OUTPUT / "hw" / slug
        target = ROOT / "target/homework-docs" / slug
        print(f"Building homework: {slug}", flush=True)

        subprocess.run(
            ["cargo", "doc", "--no-deps", "--manifest-path", source / "Cargo.toml",
             "--target-dir", target],
            cwd=ROOT, check=True,
        )
        require_file(target / "doc" / slug / "index.html")
        shutil.copytree(target / "doc", destination / "doc")
        archive = destination / f"{slug}.zip"
        create_archive(source, archive)
        require_file(archive, b"PK")

    for lecture in LECTURES:
        slug = lecture[3:]
        destination = OUTPUT / "lectures" / lecture
        site = destination / "deck"
        runner = ["node", "scripts/run.mjs"]
        environment = os.environ | {
            "STUCO_SLIDEV_SITE_OUTPUT": str(site),
            "STUCO_SLIDEV_PDF_OUTPUT": str(destination),
        }
        print(f"Building lecture: {lecture}", flush=True)

        subprocess.run(
            [*runner, "build", lecture], cwd=SLIDEV, env=environment, check=True,
        )
        require_file(site / "index.html")
        # Cloudflare rejects the catch-all redirect that Slidev emits for each deck.
        (site / "_redirects").unlink(missing_ok=True)

        # Export sequentially, with one retry for transient browser failures.
        for theme in ["light", "dark"]:
            command = [*runner, f"export:{theme}", lecture]
            try:
                subprocess.run(command, cwd=SLIDEV, env=environment, check=True)
            except subprocess.CalledProcessError:
                print(f"Retrying {lecture} {theme} PDF export", flush=True)
                subprocess.run(command, cwd=SLIDEV, env=environment, check=True)

            require_file(destination / f"{slug}-{theme}.pdf", b"%PDF-")

    print(f"Built complete site: {OUTPUT}", flush=True)


if __name__ == "__main__":
    main()
