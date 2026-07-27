use crate::utils;
use std::{fs, io, path::Path, process::Command};

/// The deployment base that `scripts/run.mjs` passes to Slidev, relative to `public/`.
const SITE_OUTPUT: &str = "public/slidev/week01";

/// The rewrite file Slidev generates for its client-side routes, which Cloudflare rejects.
const REDIRECTS: &str = "_redirects";

/// Renders the Slidev week-one deck into `public/` so deployments serve it beside the marp PDFs.
///
/// Slidev is still an experiment, so this stage renders the one deck that has been converted rather
/// than replacing [`crate::lectures`]. Both stages read the same lecture sources.
pub fn build(manifest_dir: &Path) -> io::Result<()> {
    let slidev_root = manifest_dir.join("slidev");
    let output_dir = manifest_dir.join(SITE_OUTPUT);
    let index = output_dir.join("index.html");

    utils::require_directory(&slidev_root)?;
    utils::require_nonempty_file(&slidev_root.join("package.json"))?;

    // The deck and its images live outside `slidev/`, so watch them alongside the toolchain.
    let mut dependencies = utils::files_in_tree(&slidev_root)?;
    for directory in ["lectures/01_introduction", "lectures/images"] {
        dependencies.extend(utils::files_in_tree(&manifest_dir.join(directory))?);
    }

    if utils::generated_files_are_current(&dependencies, &[index.as_path()]) {
        return Ok(());
    }

    install_toolchain(&slidev_root)?;

    // Slidev writes into the directory without clearing it, so drop earlier output first.
    utils::recreate_directory(&output_dir)?;

    let mut command = Command::new("npm");
    command
        .arg("run")
        .arg("build")
        .env("STUCO_SLIDEV_SITE_OUTPUT", &output_dir)
        .current_dir(&slidev_root);
    utils::run_command(command)?;

    utils::require_nonempty_file(&index)?;

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

/// Installs the pinned Slidev toolchain.
///
/// Only PDF export drives a browser, so the site build skips Playwright's browser download rather
/// than omitting optional dependencies, which Rolldown needs for its platform-specific binding.
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
