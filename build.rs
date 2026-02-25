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
    ("homeworks/week12/filterlab", "filterlab"),
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
            println!("cargo:warning=Lecture not found: {}", md_file.display());
            return;
        }

        let topic_out_dir = out_dir.join(dir_name);
        fs::create_dir_all(&topic_out_dir).ok();

        let dark_pdf = topic_out_dir.join(format!("{topic}-dark.pdf"));
        let light_pdf = topic_out_dir.join(format!("{topic}-light.pdf"));

        // Skip if up to date
        if !needs_rebuild(&md_file, &[&dark_pdf, &light_pdf]) {
            return;
        }

        let config = lectures_dir.join("marp_config.json");

        // Render dark theme
        render_marp(&md_file, &dark_pdf, &config, &src_dir);

        // Render light theme
        let content = fs::read_to_string(&md_file).expect("Failed to read markdown");
        let light_content = content.replace("class: invert", "# class: invert");
        let temp_light = src_dir.join(format!("{topic}-light-temp.md"));
        fs::write(&temp_light, &light_content).expect("Failed to write temp file");

        render_marp(&temp_light, &light_pdf, &config, &src_dir);
        fs::remove_file(&temp_light).ok();

        println!("cargo:warning=Rendered {topic}");
    });
}

fn render_marp(input: &Path, output: &Path, config: &Path, working_dir: &Path) {
    let status = Command::new("marp")
        .arg(input.file_name().unwrap())
        .arg("-c")
        .arg(config)
        .arg("-o")
        .arg(output)
        .current_dir(working_dir)
        .status();

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => println!("cargo:warning=marp exited with: {:?}", s),
        Err(e) => println!(
            "cargo:warning=Failed to run marp: {e} (kind: {:?})",
            e.kind()
        ),
    }
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
        if Path::new(path).exists() {
            let out_dir = format!("public/hw/{slug}");
            fs::create_dir_all(&out_dir).ok();

            // Create zip handout
            let zip_path = format!("{out_dir}/{slug}.zip");
            if let Err(e) = create_zip(path, &zip_path, slug) {
                println!("cargo:warning=Failed to zip {slug}: {e}");
            }

            // Generate docs
            let manifest = format!("{path}/Cargo.toml");
            let status = Command::new("cargo")
                .args([
                    "doc",
                    "--no-deps",
                    "--manifest-path",
                    &manifest,
                    "--target-dir",
                    &out_dir,
                ])
                .status();

            match status {
                Ok(s) if s.success() => {}
                Ok(_) => println!("cargo:warning=cargo doc failed for {slug}"),
                Err(e) => println!("cargo:warning=Failed to run cargo doc for {slug}: {e}"),
            }
        }
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
}

fn main() {
    println!("cargo:rerun-if-changed=src/syllabus.typ");

    // We can't just use "cargo:rerun-if-changed=homeworks" because we'd be
    // recursively rebuilding over and over due to Cargo.lock and handin.zip
    watch_homeworks();
    watch_lectures();

    // Create public dir if it doesn't exist
    std::fs::create_dir_all("public").ok();

    // Build everything in parallel
    rayon::join(
        || rayon::join(build_syllabus, build_lectures),
        build_homeworks,
    );
}

fn create_zip(src_dir: &str, zip_path: &str, root_name: &str) -> io::Result<()> {
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
