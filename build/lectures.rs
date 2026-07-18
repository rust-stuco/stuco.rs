use crate::utils;
use rayon::prelude::*;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

struct Lecture {
    directory: &'static str,
    slug: &'static str,
}

impl Lecture {
    const fn new(directory: &'static str, slug: &'static str) -> Self {
        Self { directory, slug }
    }
}

const LECTURES: &[Lecture] = &[
    Lecture::new("01_introduction", "introduction"),
    Lecture::new("02_ownership_p1", "ownership_p1"),
    Lecture::new("03_structs_enums", "structs_enums"),
    Lecture::new("04_collections_generics", "collections_generics"),
    Lecture::new("05_errors_traits", "errors_traits"),
    Lecture::new("06_modules_testing", "modules_testing"),
    Lecture::new("07_ecosystem", "ecosystem"),
    Lecture::new("08_closures_iterators", "closures_iterators"),
    Lecture::new("09_ownership_p2", "ownership_p2"),
    Lecture::new("10_lifetimes", "lifetimes"),
    Lecture::new("11_smart_pointers", "smart_pointers"),
    Lecture::new("12_unsafe", "unsafe"),
    Lecture::new("13_parallelism", "parallelism"),
    Lecture::new("14_concurrency", "concurrency"),
];

pub fn build(manifest_dir: &Path) -> io::Result<()> {
    let source_root = manifest_dir.join("lectures");
    let output_root = manifest_dir.join("public/lectures");
    let config = source_root.join("marp_config.json");
    let theme = source_root.join("styles/rust.css");

    utils::require_nonempty_file(&config)?;
    utils::require_nonempty_file(&theme)?;

    let mut shared_inputs = vec![config.clone(), theme];
    for directory in ["fonts", "images", "styles"] {
        let path = source_root.join(directory);
        if path.exists() {
            shared_inputs.extend(utils::files_in_tree(&path)?);
        }
    }

    LECTURES.par_iter().try_for_each(|lecture| {
        build_lecture(lecture, &source_root, &output_root, &config, &shared_inputs)
    })
}

fn build_lecture(
    lecture: &Lecture,
    source_root: &Path,
    output_root: &Path,
    config: &Path,
    shared_inputs: &[PathBuf],
) -> io::Result<()> {
    let source_dir = source_root.join(lecture.directory);
    let source = source_dir.join(format!("{}.md", lecture.slug));

    let output_dir = output_root.join(lecture.directory);
    let dark_pdf = output_dir.join(format!("{}-dark.pdf", lecture.slug));
    let light_pdf = output_dir.join(format!("{}-light.pdf", lecture.slug));

    utils::require_nonempty_file(&source)?;
    utils::create_directory(&output_dir)?;

    let mut dependencies = utils::files_in_tree(&source_dir)?;
    dependencies.extend_from_slice(shared_inputs);

    let generated_files = [dark_pdf.as_path(), light_pdf.as_path()];
    if utils::generated_files_are_current(&dependencies, &generated_files) {
        return Ok(());
    }

    render_marp(&source, &dark_pdf, config, &source_dir)?;
    render_light_marp(&source, &light_pdf, config, &source_dir)?;

    utils::require_nonempty_file(&dark_pdf)?;
    utils::require_nonempty_file(&light_pdf)?;

    println!("cargo:warning=Rendered {}", lecture.slug);
    Ok(())
}

/// Renders a light-mode PDF by commenting out the dark-mode directive in a temporary source copy.
fn render_light_marp(
    input: &Path,
    output: &Path,
    config: &Path,
    working_dir: &Path,
) -> io::Result<()> {
    let file_stem = input.file_stem().ok_or_else(|| {
        io::Error::other(format!(
            "lecture source {} has no file stem",
            input.display()
        ))
    })?;
    let temporary_source =
        input.with_file_name(format!("{}-light-temp.md", file_stem.to_string_lossy()));

    let markdown = fs::read_to_string(input).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("failed to read lecture source {}: {error}", input.display()),
        )
    })?;
    fs::write(
        &temporary_source,
        markdown.replace("class: invert", "# class: invert"),
    )
    .map_err(|error| {
        io::Error::new(
            error.kind(),
            format!(
                "failed to write temporary lecture source {}: {error}",
                temporary_source.display()
            ),
        )
    })?;

    // Always try to remove the temporary source, even when Marp fails.
    let render_result = render_marp(&temporary_source, output, config, working_dir);
    let cleanup_result = fs::remove_file(&temporary_source).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!(
                "failed to remove temporary lecture source {}: {error}",
                temporary_source.display()
            ),
        )
    });
    render_result?;
    cleanup_result
}

fn render_marp(input: &Path, output: &Path, config: &Path, working_dir: &Path) -> io::Result<()> {
    let file_name = input.file_name().ok_or_else(|| {
        io::Error::other(format!(
            "lecture source {} has no file name",
            input.display()
        ))
    })?;

    let mut command = Command::new("marp");
    command
        .arg(file_name)
        .arg("-c")
        .arg(config)
        .arg("-o")
        .arg(output)
        .current_dir(working_dir);
    utils::run_command(command)
}
