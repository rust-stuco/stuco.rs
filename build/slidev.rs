use crate::utils;
use std::{env, ffi::OsStr, fs, io, path::Path, process::Command, slice};

/// Where the built deck lands, holding one site per color scheme: `light/` and `dark/`.
///
/// Deliberately outside `public/`: `dx` runs every JavaScript file it finds there through its asset
/// pipeline, which re-bundles each of Slidev's chunks into a standalone copy of the whole deck. The
/// module graph does not survive that, and the deck renders as a blank page. The deployment overlays
/// this directory onto the bundle afterwards instead.
const SITE_OUTPUT: &str = "target/slidev/week01";

/// Where the schedule page expects the week-one slide PDFs.
const PDF_OUTPUT: &str = "public/lectures/01_introduction";

/// The rewrite file Slidev generates for its client-side routes, which Cloudflare rejects.
const REDIRECTS: &str = "_redirects";

/// The color schemes the deck is built for, each its own site under [`SITE_OUTPUT`].
const LIGHT: &str = "light";
const DARK: &str = "dark";

/// Renders the Slidev week-one deck, as both a browsable site and the slide PDFs.
///
/// Slidev is still an experiment, so this stage owns the one deck that has been converted while
/// [`crate::lectures`] renders the rest with marp. The PDFs go straight into `public/`, since `dx`
/// copies binary files through untouched; the deck itself cannot, as [`SITE_OUTPUT`] explains.
pub fn build(manifest_dir: &Path) -> io::Result<()> {
    let slidev_root = manifest_dir.join("slidev");
    let output_dir = manifest_dir.join(SITE_OUTPUT);
    let light_index = output_dir.join("light/index.html");
    let dark_index = output_dir.join("dark/index.html");
    let pdf_dir = manifest_dir.join(PDF_OUTPUT);
    let light_pdf = pdf_dir.join("introduction-light.pdf");
    let dark_pdf = pdf_dir.join("introduction-dark.pdf");

    utils::require_directory(&slidev_root)?;
    utils::require_nonempty_file(&slidev_root.join("package.json"))?;

    // Only what the render actually reads, so editing this directory's documentation does not cost a
    // reinstall and two PDF exports. The deck and its images live outside `slidev/`.
    let mut dependencies = vec![
        slidev_root.join("package.json"),
        slidev_root.join("package-lock.json"),
    ];
    for directory in ["runtime", "scripts"] {
        dependencies.extend(utils::files_in_tree(&slidev_root.join(directory))?);
    }
    for directory in ["lectures/01_introduction", "lectures/images"] {
        dependencies.extend(utils::files_in_tree(&manifest_dir.join(directory))?);
    }

    let generated_files = [
        light_index.as_path(),
        dark_index.as_path(),
        light_pdf.as_path(),
        dark_pdf.as_path(),
    ];
    if utils::generated_files_are_current(&dependencies, &generated_files) {
        // Still mirror: the render is current, but the bundle may have been cleaned since.
        return mirror_into_bundle(manifest_dir, &output_dir);
    }

    install_toolchain(&slidev_root)?;

    // Slidev writes into the directory without clearing it, so drop earlier output first.
    utils::recreate_directory(&output_dir)?;
    utils::create_directory(&pdf_dir)?;

    // Slidev pins the color scheme at build time, so each one is a separate site, mirroring the two
    // PDFs the schedule page links.
    for schema in [LIGHT, DARK] {
        let variant = output_dir.join(schema);
        run_task(
            &slidev_root,
            "build",
            &[
                ("STUCO_SLIDEV_SITE_OUTPUT", variant.as_os_str()),
                ("STUCO_SLIDEV_COLOR_SCHEMA", OsStr::new(schema)),
            ],
        )?;
        utils::require_nonempty_file(&variant.join("index.html"))?;
        remove_generated_redirects(&variant)?;
    }

    // Exporting drives a browser, so run the two color schemes sequentially rather than in parallel.
    for task in ["export:light", "export:dark"] {
        run_task(
            &slidev_root,
            task,
            &[("STUCO_SLIDEV_PDF_OUTPUT", pdf_dir.as_os_str())],
        )?;
    }
    utils::require_nonempty_file(&light_pdf)?;
    utils::require_nonempty_file(&dark_pdf)?;

    mirror_into_bundle(manifest_dir, &output_dir)?;

    println!("cargo:warning=Rendered the Slidev week-one deck");
    Ok(())
}

/// Copies the deck into the bundle `dx` serves, so `dx serve` offers the same paths a deployment does.
///
/// The layout below belongs to `dx` rather than to us, and it is written before `dx` has staged
/// anything, so treat it as a convenience only: the deploy workflow overlays the deck explicitly,
/// and that is what production depends on.
fn mirror_into_bundle(manifest_dir: &Path, site: &Path) -> io::Result<()> {
    let Some(profile) = env::var_os("PROFILE") else {
        return Ok(());
    };

    let destination = manifest_dir
        .join("target/dx")
        .join(env!("CARGO_PKG_NAME"))
        .join(profile)
        .join("web/public/slidev/week01");

    utils::recreate_directory(&destination)?;
    utils::copy_directory(site, &destination)
}

/// Removes the rewrite Slidev generates, which points its own wildcard back at `index.html`.
///
/// Cloudflare rejects that rule as an infinite loop, and hash routing leaves nothing that needs it.
fn remove_generated_redirects(site: &Path) -> io::Result<()> {
    let redirects = site.join(REDIRECTS);

    match fs::remove_file(&redirects) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(io::Error::new(
            error.kind(),
            format!("failed to remove {}: {error}", redirects.display()),
        )),
    }
}

/// Runs one of the `slidev/` npm scripts with the environment it needs to write into the tree.
fn run_task(slidev_root: &Path, task: &str, environment: &[(&str, &OsStr)]) -> io::Result<()> {
    let mut command = Command::new("npm");
    command.arg("run").arg(task).current_dir(slidev_root);
    for (name, value) in environment {
        command.env(name, value);
    }

    utils::run_command(command)
}

/// Installs the Slidev toolchain exactly as `package-lock.json` records it, if it is not already.
///
/// `npm ci` replaces `node_modules` wholesale, so running it on every render is both slow and a way
/// to lose the tree to an interrupted build. npm writes its own marker once an install completes, so
/// comparing that against the lockfile skips the work and still repairs a half-finished install.
///
/// Export renders through whatever browser is already installed, the way the marp config does, so
/// skip Playwright's own download. Optional dependencies stay in, since Rolldown ships its
/// platform-specific binding as one.
fn install_toolchain(slidev_root: &Path) -> io::Result<()> {
    let lockfile = slidev_root.join("package-lock.json");
    let installed = slidev_root.join("node_modules/.package-lock.json");

    utils::require_nonempty_file(&lockfile)?;
    if utils::generated_files_are_current(slice::from_ref(&lockfile), &[installed.as_path()]) {
        return Ok(());
    }

    let mut command = Command::new("npm");
    command
        .arg("ci")
        .arg("--no-audit")
        .arg("--no-fund")
        .env("PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD", "1")
        .current_dir(slidev_root);
    utils::run_command(command)
}
