use ignore::Walk;
use rayon::prelude::*;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};
use zip::{ZipWriter, write::SimpleFileOptions};

const LECTURES: &[(&str, &str)] = &[
    ("01_introduction", "introduction"),
    ("02_ownership_p1", "ownership_p1"),
    ("03_structs_enums", "structs_enums"),
    ("04_collections_generics", "collections_generics"),
    ("05_errors_traits", "errors_traits"),
    ("06_modules_testing", "modules_testing"),
    ("07_ecosystem", "ecosystem"),
    ("08_closures_iterators", "closures_iterators"),
    ("09_ownership_p2", "ownership_p2"),
    ("10_lifetimes", "lifetimes"),
    ("11_smart_pointers", "smart_pointers"),
    ("12_unsafe", "unsafe"),
    ("13_parallelism", "parallelism"),
    ("14_concurrency", "concurrency"),
];

const HOMEWORKS: &[(&str, &str)] = &[
    ("homeworks/week1/primerlab", "primerlab"),
    ("homeworks/week2/getownedlab", "getownedlab"),
    ("homeworks/week3/cardlab", "cardlab"),
    ("homeworks/week4/multilab", "multilab"),
    ("homeworks/week5/pokerlab", "pokerlab"),
    ("homeworks/week5-ec/summarylab", "summarylab"),
    ("homeworks/week6/greplab", "greplab"),
    ("homeworks/week8/iterlab", "iterlab"),
    ("homeworks/week10/splitlab", "splitlab"),
    ("homeworks/week11/filterlab", "filterlab"),
    ("homeworks/week13/rowlab", "rowlab"),
];

fn watch_lectures() {
    for entry in Walk::new("lectures").flatten() {
        if entry.path().is_file() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }
}

fn needs_rebuild(src: &Path, outputs: &[&Path]) -> bool {
    let Ok(src_meta) = fs::metadata(src) else {
        return true;
    };
    let Ok(src_mtime) = src_meta.modified() else {
        return true;
    };

    for output in outputs {
        match fs::metadata(output) {
            Ok(meta) => {
                if meta.modified().map(|t| t < src_mtime).unwrap_or(true) {
                    return true;
                }
            }
            Err(_) => return true,
        }
    }
    false
}

fn build_lectures() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let lectures_dir = manifest_dir.join("lectures");
    let out_dir = manifest_dir.join("public/lectures");

    LECTURES.par_iter().for_each(|(dir_name, topic)| {
        let src_dir = lectures_dir.join(dir_name);
        let md_file = src_dir.join(format!("{topic}.md"));

        if !md_file.exists() {
            panic!("required lecture not found: {}", md_file.display());
        }

        let topic_out_dir = out_dir.join(dir_name);
        fs::create_dir_all(&topic_out_dir).unwrap_or_else(|error| {
            panic!(
                "failed to create lecture output directory {}: {error}",
                topic_out_dir.display()
            )
        });

        let dark_pdf = topic_out_dir.join(format!("{topic}-dark.pdf"));
        let light_pdf = topic_out_dir.join(format!("{topic}-light.pdf"));

        // Skip if up to date
        if !needs_rebuild(&md_file, &[&dark_pdf, &light_pdf]) {
            return;
        }

        let config = lectures_dir.join("marp_config.json");

        // Render dark theme
        render_marp(&md_file, &dark_pdf, &config, &src_dir)
            .unwrap_or_else(|error| panic!("{error}"));

        // Render light theme
        let content = fs::read_to_string(&md_file).expect("Failed to read markdown");
        let light_content = content.replace("class: invert", "# class: invert");
        let temp_light = src_dir.join(format!("{topic}-light-temp.md"));
        fs::write(&temp_light, &light_content).expect("Failed to write temp file");

        let light_result = render_marp(&temp_light, &light_pdf, &config, &src_dir);
        let cleanup_result = fs::remove_file(&temp_light);

        light_result.unwrap_or_else(|error| panic!("{error}"));
        cleanup_result.unwrap_or_else(|error| {
            panic!(
                "failed to remove temporary lecture source {}: {error}",
                temp_light.display()
            )
        });

        require_nonempty_file(&dark_pdf);
        require_nonempty_file(&light_pdf);

        println!("cargo:warning=Rendered {topic}");
    });
}

fn render_marp(
    input: &Path,
    output: &Path,
    config: &Path,
    working_dir: &Path,
) -> Result<(), String> {
    let status = Command::new("marp")
        .arg(input.file_name().unwrap())
        .arg("-c")
        .arg(config)
        .arg("-o")
        .arg(output)
        .current_dir(working_dir)
        .status()
        .map_err(|error| {
            format!(
                "failed to run Marp for {}: {error} (kind: {:?})",
                input.display(),
                error.kind()
            )
        })?;

    if !status.success() {
        return Err(format!(
            "Marp failed for {} with status {status}",
            input.display()
        ));
    }

    Ok(())
}

fn watch_homeworks() {
    for entry in Walk::new("homeworks").flatten() {
        let path = entry.path();
        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn build_homeworks() {
    HOMEWORKS.par_iter().for_each(|(path, slug)| {
        let homework_path = Path::new(path);
        if !homework_path.exists() {
            panic!("required homework not found: {}", homework_path.display());
        }

        let out_dir = PathBuf::from(format!("public/hw/{slug}"));
        fs::create_dir_all(&out_dir).unwrap_or_else(|error| {
            panic!(
                "failed to create homework output directory {}: {error}",
                out_dir.display()
            )
        });

        // Create zip handout
        let zip_path = out_dir.join(format!("{slug}.zip"));
        create_zip(path, &zip_path, slug)
            .unwrap_or_else(|error| panic!("failed to zip {slug}: {error}"));

        // Generate docs
        let manifest = format!("{path}/Cargo.toml");
        let status = Command::new("cargo")
            .args([
                "doc",
                "--no-deps",
                "--manifest-path",
                &manifest,
                "--target-dir",
            ])
            .arg(&out_dir)
            .status()
            .unwrap_or_else(|error| panic!("failed to run cargo doc for {slug}: {error}"));

        if !status.success() {
            panic!("cargo doc failed for {slug} with status {status}");
        }

        require_nonempty_file(&zip_path);
        require_nonempty_file(&out_dir.join("doc").join(slug).join("index.html"));
    });
}

fn build_syllabus() {
    let status = Command::new("typst")
        .args(["compile", "src/syllabus.typ", "public/syllabus.pdf"])
        .status()
        .expect("failed to run typst");

    if !status.success() {
        panic!("typst compilation failed");
    }

    require_nonempty_file(Path::new("public/syllabus.pdf"));
}

fn main() {
    println!("cargo:rerun-if-changed=src/syllabus.typ");

    // We can't just use "cargo:rerun-if-changed=homeworks" because we'd be
    // recursively rebuilding over and over due to Cargo.lock and handin.zip
    watch_homeworks();
    watch_lectures();

    // Create public dir if it doesn't exist
    std::fs::create_dir_all("public").expect("failed to create public output directory");

    // Build everything in parallel
    rayon::join(
        || rayon::join(build_syllabus, build_lectures),
        build_homeworks,
    );
}

fn require_nonempty_file(path: &Path) {
    let metadata = fs::metadata(path)
        .unwrap_or_else(|error| panic!("required output {} is missing: {error}", path.display()));

    assert!(
        metadata.is_file() && metadata.len() > 0,
        "required output {} is not a non-empty file",
        path.display()
    );
}

fn create_zip(src_dir: &str, zip_path: &Path, root_name: &str) -> io::Result<()> {
    let file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let src_path = Path::new(src_dir);
    add_dir_to_zip(&mut zip, src_path, root_name, &options)?;

    zip.finish()?;
    Ok(())
}

fn add_dir_to_zip(
    zip: &mut ZipWriter<File>,
    dir: &Path,
    prefix: &str,
    options: &SimpleFileOptions,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        // Skip target directory and hidden files
        if name == "target" || name.starts_with('.') {
            continue;
        }

        let zip_path = format!("{prefix}/{name}");

        if path.is_dir() {
            zip.add_directory(&zip_path, *options)?;
            add_dir_to_zip(zip, &path, &zip_path, options)?;
        } else {
            zip.start_file(&zip_path, *options)?;
            let contents = fs::read(&path)?;
            zip.write_all(&contents)?;
        }
    }
    Ok(())
}
