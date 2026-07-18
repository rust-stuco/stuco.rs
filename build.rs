use std::{
    env, io,
    path::{Path, PathBuf},
    process::Command,
    slice,
};

#[path = "build/homeworks.rs"]
mod homeworks;
#[path = "build/lectures.rs"]
mod lectures;
#[path = "build/utils.rs"]
mod utils;

fn main() -> io::Result<()> {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("CARGO_MANIFEST_DIR is not set"))?;

    println!("cargo:rerun-if-changed=src/syllabus.typ");

    // Watching individual files avoids rebuild loops from ignored homework artifacts.
    emit_rerun_directives(&manifest_dir.join("homeworks"))?;
    emit_rerun_directives(&manifest_dir.join("lectures"))?;

    utils::create_directory(&manifest_dir.join("public"))?;

    // Run stages sequentially so a failure prevents later work from starting.
    build_syllabus(&manifest_dir)?;
    lectures::build(&manifest_dir)?;
    homeworks::build(&manifest_dir)
}

fn build_syllabus(manifest_dir: &Path) -> io::Result<()> {
    let source = manifest_dir.join("src/syllabus.typ");
    let output = manifest_dir.join("public/syllabus.pdf");

    if utils::generated_files_are_current(slice::from_ref(&source), &[&output]) {
        return Ok(());
    }

    let mut command = Command::new("typst");
    command.arg("compile").arg(&source).arg(&output);
    utils::run_command(command)?;

    utils::require_nonempty_file(&output)
}

fn emit_rerun_directives(root: &Path) -> io::Result<()> {
    for path in utils::files_in_tree(root)? {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    Ok(())
}
