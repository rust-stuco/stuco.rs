use crate::utils;
use rayon::prelude::*;
use std::{fs::File, io, path::Path, process::Command};
use zip::{ZipWriter, write::SimpleFileOptions};

struct Homework {
    source: &'static str,
    slug: &'static str,
}

impl Homework {
    const fn new(source: &'static str, slug: &'static str) -> Self {
        Self { source, slug }
    }
}

const HOMEWORKS: &[Homework] = &[
    Homework::new("homeworks/week1/primerlab", "primerlab"),
    Homework::new("homeworks/week2/getownedlab", "getownedlab"),
    Homework::new("homeworks/week3/cardlab", "cardlab"),
    Homework::new("homeworks/week4/multilab", "multilab"),
    Homework::new("homeworks/week5/pokerlab", "pokerlab"),
    Homework::new("homeworks/week5-ec/summarylab", "summarylab"),
    Homework::new("homeworks/week6/greplab", "greplab"),
    Homework::new("homeworks/week8/iterlab", "iterlab"),
    Homework::new("homeworks/week10/splitlab", "splitlab"),
    Homework::new("homeworks/week11/filterlab", "filterlab"),
    Homework::new("homeworks/week13/rowlab", "rowlab"),
];

pub fn build(manifest_dir: &Path) -> io::Result<()> {
    HOMEWORKS
        .par_iter()
        .try_for_each(|homework| build_homework(homework, manifest_dir))
}

fn build_homework(homework: &Homework, manifest_dir: &Path) -> io::Result<()> {
    let source = manifest_dir.join(homework.source);
    let output_dir = manifest_dir.join("public/hw").join(homework.slug);
    let target_dir = manifest_dir
        .join("target/homework-docs")
        .join(homework.slug);

    utils::require_directory(&source)?;

    let manifest = source.join("Cargo.toml");
    let mut command = Command::new("cargo");
    command
        // Dioxus sets WASM-specific Rust flags that are invalid for native documentation builds.
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .arg("doc")
        .arg("--no-deps")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--target-dir")
        .arg(&target_dir);
    utils::run_command(command)?;

    let generated_documentation = target_dir
        .join("doc")
        .join(homework.slug)
        .join("index.html");
    utils::require_nonempty_file(&generated_documentation)?;

    utils::recreate_directory(&output_dir)?;
    utils::copy_directory(&target_dir.join("doc"), &output_dir.join("doc"))?;

    let archive = output_dir.join(format!("{}.zip", homework.slug));
    create_zip(&source, &archive, homework.slug).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!(
                "failed to create homework archive {}: {error}",
                archive.display()
            ),
        )
    })?;
    utils::require_nonempty_file(&archive)
}

fn create_zip(source: &Path, output: &Path, root_name: &str) -> io::Result<()> {
    let file = File::create(output)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for source_file in utils::files_in_tree(source)? {
        let relative_path = source_file
            .strip_prefix(source)
            .map_err(|error| io::Error::other(error.to_string()))?;
        let archive_path = Path::new(root_name).join(relative_path);
        let archive_path = archive_path.to_string_lossy().replace('\\', "/");

        zip.start_file(archive_path, options)?;
        io::copy(&mut File::open(source_file)?, &mut zip)?;
    }

    zip.finish()?;
    Ok(())
}
