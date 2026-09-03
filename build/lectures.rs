//! Builds every lecture as a Slidev site and as light and dark PDFs.
//!
//! The [`build`] entry point renders stale decks, validates their outputs, and mirrors each site
//! into the directory that `dx serve` exposes.

use crate::utils;
use rayon::{ThreadPoolBuilder, prelude::*};
use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
    process::Command,
    slice,
};

/// Where the built lecture sites land, with one `deck/` directory per lecture.
///
/// Deliberately outside `public/`: `dx` runs every JavaScript file it finds there through its asset
/// pipeline, which re-bundles each of Slidev's chunks into a standalone copy of the whole deck. The
/// module graph does not survive that, so deployment overlays these sites onto the bundle later.
const SITE_OUTPUT: &str = "target/slidev/lectures";

/// Serializes Cargo build scripts that publish into the shared Slidev output directories.
const BUILD_LOCK: &str = "target/slidev/.build.lock";

/// Where the schedule page expects the lecture PDFs.
const PDF_OUTPUT: &str = "public/lectures";

/// The rewrite file Slidev generates for client-side routes, which Cloudflare rejects.
const REDIRECTS: &str = "_redirects";

/// Slidev's exporter starts Chrome and is unreliable when two lecture pipelines compete for the
/// CI runner, so render one lecture at a time.
const MAX_PARALLEL_LECTURES: usize = 1;

#[derive(Clone, Copy)]
struct Lecture {
    directory: &'static str,
    slug: &'static str,
}

impl Lecture {
    const fn new(directory: &'static str, slug: &'static str) -> Self {
        Self { directory, slug }
    }

    fn source(self, manifest_dir: &Path) -> PathBuf {
        manifest_dir
            .join("lectures")
            .join(self.directory)
            .join(format!("{}.md", self.slug))
    }

    fn site(self, manifest_dir: &Path) -> PathBuf {
        manifest_dir
            .join(SITE_OUTPUT)
            .join(self.directory)
            .join("deck")
    }

    fn pdf_directory(self, manifest_dir: &Path) -> PathBuf {
        manifest_dir.join(PDF_OUTPUT).join(self.directory)
    }

    fn light_pdf(self, manifest_dir: &Path) -> PathBuf {
        self.pdf_directory(manifest_dir)
            .join(format!("{}-light.pdf", self.slug))
    }

    fn dark_pdf(self, manifest_dir: &Path) -> PathBuf {
        self.pdf_directory(manifest_dir)
            .join(format!("{}-dark.pdf", self.slug))
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

/// Renders every published lecture as an interactive Slidev site and two PDFs.
pub fn build(manifest_dir: &Path) -> io::Result<()> {
    let build_lock_path = manifest_dir.join(BUILD_LOCK);
    utils::create_directory(
        build_lock_path
            .parent()
            .expect("BUILD_LOCK includes a parent directory"),
    )?;
    let build_lock = File::create(&build_lock_path)?;
    build_lock.lock().map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("failed to lock shared Slidev outputs: {error}"),
        )
    })?;

    let slidev_root = manifest_dir.join("slidev");
    utils::require_directory(&slidev_root)?;
    utils::require_nonempty_file(&slidev_root.join("package.json"))?;

    let shared_dependencies = shared_dependencies(manifest_dir, &slidev_root)?;
    let stale_lectures = LECTURES
        .iter()
        .copied()
        .filter(|lecture| !outputs_are_current(*lecture, manifest_dir, &shared_dependencies))
        .collect::<Vec<_>>();

    if !stale_lectures.is_empty() {
        install_toolchain(&slidev_root)?;

        let render_pool = ThreadPoolBuilder::new()
            .num_threads(MAX_PARALLEL_LECTURES)
            .build()
            .map_err(|error| io::Error::other(format!("failed to create render pool: {error}")))?;
        render_pool.install(|| {
            stale_lectures
                .par_iter()
                .try_for_each(|lecture| render_lecture(*lecture, manifest_dir, &slidev_root))
        })?;
    }

    for lecture in LECTURES {
        require_outputs(*lecture, manifest_dir)?;
        mirror_into_bundle(*lecture, manifest_dir)?;
    }

    Ok(())
}

fn shared_dependencies(manifest_dir: &Path, slidev_root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut dependencies = vec![
        slidev_root.join("package.json"),
        slidev_root.join("package-lock.json"),
    ];
    for directory in ["runtime", "scripts"] {
        dependencies.extend(utils::files_in_tree(&slidev_root.join(directory))?);
    }
    dependencies.extend(utils::files_in_tree(&manifest_dir.join("lectures/images"))?);

    Ok(dependencies)
}

fn outputs_are_current(
    lecture: Lecture,
    manifest_dir: &Path,
    shared_dependencies: &[PathBuf],
) -> bool {
    let source_directory = manifest_dir.join("lectures").join(lecture.directory);
    let mut dependencies = shared_dependencies.to_vec();
    let Ok(local_dependencies) = utils::files_in_tree(&source_directory) else {
        return false;
    };
    dependencies.extend(local_dependencies);

    let index = lecture.site(manifest_dir).join("index.html");
    let light_pdf = lecture.light_pdf(manifest_dir);
    let dark_pdf = lecture.dark_pdf(manifest_dir);
    let generated_files = [index.as_path(), light_pdf.as_path(), dark_pdf.as_path()];

    utils::generated_files_are_current(&dependencies, &generated_files)
        && pdf_has_signature(&light_pdf)
        && pdf_has_signature(&dark_pdf)
}

fn render_lecture(lecture: Lecture, manifest_dir: &Path, slidev_root: &Path) -> io::Result<()> {
    let source = lecture.source(manifest_dir);
    let site = lecture.site(manifest_dir);
    let pdf_directory = lecture.pdf_directory(manifest_dir);

    utils::require_nonempty_file(&source)?;
    utils::recreate_directory(&site)?;
    utils::create_directory(&pdf_directory)?;

    run_task(
        slidev_root,
        lecture,
        "build",
        &[("STUCO_SLIDEV_SITE_OUTPUT", site.as_os_str())],
    )?;
    utils::require_nonempty_file(&site.join("index.html"))?;
    remove_generated_redirects(&site)?;

    // Run each lecture's browser exports sequentially. The render pool also serializes lectures, so
    // only one browser process runs at once.
    for task in ["export:light", "export:dark"] {
        let environment = [("STUCO_SLIDEV_PDF_OUTPUT", pdf_directory.as_os_str())];
        if let Err(first_error) = run_task(slidev_root, lecture, task, &environment) {
            println!(
                "cargo:warning=Retrying {task} for {} after: {first_error}",
                lecture.slug
            );
            run_task(slidev_root, lecture, task, &environment).map_err(|second_error| {
                io::Error::other(format!(
                    "{task} failed twice for {}: {first_error}; retry: {second_error}",
                    lecture.slug
                ))
            })?;
        }
    }
    require_pdf(&lecture.light_pdf(manifest_dir))?;
    require_pdf(&lecture.dark_pdf(manifest_dir))?;

    println!("cargo:warning=Rendered {}", lecture.slug);
    Ok(())
}

/// Copies one deck into the bundle `dx` serves without disturbing its sibling PDFs.
///
/// The layout below belongs to `dx` rather than to us, and it is written before `dx` has staged
/// everything, so deployment overlays the sites explicitly after `dx build` as the authoritative
/// copy. This earlier copy makes the same paths available from `dx serve`.
fn mirror_into_bundle(lecture: Lecture, manifest_dir: &Path) -> io::Result<()> {
    let Some(profile) = env::var_os("PROFILE") else {
        return Ok(());
    };

    let destination = manifest_dir
        .join("target/dx")
        .join(env!("CARGO_PKG_NAME"))
        .join(profile)
        .join("web/public/lectures")
        .join(lecture.directory)
        .join("deck");

    utils::recreate_directory(&destination)?;
    utils::copy_directory(&lecture.site(manifest_dir), &destination)
}

fn require_outputs(lecture: Lecture, manifest_dir: &Path) -> io::Result<()> {
    utils::require_nonempty_file(&lecture.site(manifest_dir).join("index.html"))?;
    require_pdf(&lecture.light_pdf(manifest_dir))?;
    require_pdf(&lecture.dark_pdf(manifest_dir))
}

fn pdf_has_signature(path: &Path) -> bool {
    let Ok(mut file) = File::open(path) else {
        return false;
    };
    let mut signature = [0; 5];

    file.read_exact(&mut signature).is_ok() && signature == *b"%PDF-"
}

fn require_pdf(path: &Path) -> io::Result<()> {
    if pdf_has_signature(path) {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "required PDF must begin with `%PDF-`, got {}",
            path.display()
        )))
    }
}

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

fn run_task(
    slidev_root: &Path,
    lecture: Lecture,
    task: &str,
    environment: &[(&str, &OsStr)],
) -> io::Result<()> {
    let npm = if cfg!(windows) { "npm.cmd" } else { "npm" };
    let mut command = Command::new(npm);
    command
        .arg("run")
        .arg(task)
        .arg("--")
        .arg(lecture.directory)
        // Cargo's HOST is a compiler host triple. Slidev reads it as the hostname for its local
        // export server, which cannot bind to values such as `aarch64-apple-darwin`.
        .env_remove("HOST")
        .current_dir(slidev_root);
    for (name, value) in environment {
        command.env(name, value);
    }

    utils::run_command(command)
}

/// Installs the Slidev toolchain exactly as `package-lock.json` records it, if it is not current.
///
/// npm writes its own lockfile beneath `node_modules` after a successful install. Comparing that
/// marker with the source lockfile avoids replacing a current installation and repairs an
/// interrupted one. PDF export uses the browser already installed on the system, so the install
/// skips Playwright's separate browser download.
fn install_toolchain(slidev_root: &Path) -> io::Result<()> {
    let lockfile = slidev_root.join("package-lock.json");
    let installed = slidev_root.join("node_modules/.package-lock.json");

    utils::require_nonempty_file(&lockfile)?;
    if utils::generated_files_are_current(slice::from_ref(&lockfile), &[installed.as_path()]) {
        return Ok(());
    }

    let npm = if cfg!(windows) { "npm.cmd" } else { "npm" };
    let mut command = Command::new(npm);
    command
        .arg("ci")
        .arg("--no-audit")
        .arg("--no-fund")
        .env("PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD", "1")
        .current_dir(slidev_root);
    utils::run_command(command)
}
