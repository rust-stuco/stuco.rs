use crate::utils;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

/// The deployment base that `scripts/run.mjs` passes to Slidev, relative to `public/`.
const SITE_OUTPUT: &str = "public/slidev/week01";

/// Where the schedule page expects the week-one slide PDFs.
const PDF_OUTPUT: &str = "public/lectures/01_introduction";

/// The rewrite file Slidev generates for its client-side routes, which Cloudflare rejects.
const REDIRECTS: &str = "_redirects";

/// Renders the Slidev week-one deck into `public/`, as both a browsable site and the slide PDFs.
///
/// Slidev is still an experiment, so this stage owns the one deck that has been converted while
/// [`crate::lectures`] renders the rest with marp.
pub fn build(manifest_dir: &Path) -> io::Result<()> {
    let slidev_root = manifest_dir.join("slidev");
    let output_dir = manifest_dir.join(SITE_OUTPUT);
    let index = output_dir.join("index.html");
    let pdf_dir = manifest_dir.join(PDF_OUTPUT);
    let light_pdf = pdf_dir.join("introduction-light.pdf");
    let dark_pdf = pdf_dir.join("introduction-dark.pdf");

    utils::require_directory(&slidev_root)?;
    utils::require_nonempty_file(&slidev_root.join("package.json"))?;

    // The deck and its images live outside `slidev/`, so watch them alongside the toolchain.
    let mut dependencies = utils::files_in_tree(&slidev_root)?;
    for directory in ["lectures/01_introduction", "lectures/images"] {
        dependencies.extend(utils::files_in_tree(&manifest_dir.join(directory))?);
    }

    let generated_files = [index.as_path(), light_pdf.as_path(), dark_pdf.as_path()];
    if utils::generated_files_are_current(&dependencies, &generated_files) {
        return Ok(());
    }

    install_toolchain(&slidev_root)?;

    // Slidev writes into the directory without clearing it, so drop earlier output first.
    utils::recreate_directory(&output_dir)?;
    utils::create_directory(&pdf_dir)?;

    run_task(
        &slidev_root,
        "build",
        &[("STUCO_SLIDEV_SITE_OUTPUT", &output_dir)],
    )?;
    utils::require_nonempty_file(&index)?;

    // Exporting drives a browser, so run the two color schemes sequentially rather than in parallel.
    for task in ["export:light", "export:dark"] {
        run_task(&slidev_root, task, &[("STUCO_SLIDEV_PDF_OUTPUT", &pdf_dir)])?;
    }
    utils::require_nonempty_file(&light_pdf)?;
    utils::require_nonempty_file(&dark_pdf)?;

    // Slidev writes a rewrite that points its own wildcard back at `index.html`. Cloudflare rejects
    // that as an infinite loop, and hash routing means nothing needs the rule, so drop it.
    let generated_redirects = output_dir.join(REDIRECTS);
    match fs::remove_file(&generated_redirects) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(io::Error::new(
                error.kind(),
                format!(
                    "failed to remove {}: {error}",
                    generated_redirects.display()
                ),
            ));
        }
    }

    println!("cargo:warning=Rendered the Slidev week-one deck");
    Ok(())
}

/// Runs one of the `slidev/` npm scripts with the environment it needs to write into `public/`.
fn run_task(slidev_root: &Path, task: &str, environment: &[(&str, &PathBuf)]) -> io::Result<()> {
    let mut command = Command::new("npm");
    command.arg("run").arg(task).current_dir(slidev_root);
    for (name, value) in environment {
        command.env(name, value);
    }

    utils::run_command(command)
}

/// Installs the pinned Slidev toolchain.
///
/// Export renders through whatever browser is already installed, the way the marp config does, so
/// skip Playwright's own download. Optional dependencies stay in, since Rolldown ships its
/// platform-specific binding as one.
fn install_toolchain(slidev_root: &Path) -> io::Result<()> {
    let mut command = Command::new("npm");
    command
        .arg("install")
        .arg("--no-audit")
        .arg("--no-fund")
        .env("PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD", "1")
        .current_dir(slidev_root);
    utils::run_command(command)
}
