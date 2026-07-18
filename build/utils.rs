use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

/// Returns every visible file beneath the directory that is not excluded by ignore rules.
pub fn files_in_tree(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in ignore::Walk::new(root) {
        let entry = entry.map_err(|error| {
            io::Error::other(format!("failed to inspect {}: {error}", root.display()))
        })?;

        if entry
            .file_type()
            .is_some_and(|file_type| file_type.is_file())
        {
            files.push(entry.into_path());
        }
    }

    Ok(files)
}

/// Returns whether every generated file is current, treating empty dependencies or unavailable
/// metadata as stale.
pub fn generated_files_are_current(dependencies: &[PathBuf], generated_files: &[&Path]) -> bool {
    let newest_dependency = dependencies
        .iter()
        .map(|path| {
            fs::metadata(path)
                .and_then(|metadata| metadata.modified())
                .ok()
        })
        .collect::<Option<Vec<_>>>()
        .and_then(|timestamps| timestamps.into_iter().max());

    let Some(newest_dependency) = newest_dependency else {
        return false;
    };

    generated_files.iter().all(|file| {
        let Ok(metadata) = fs::metadata(file) else {
            return false;
        };

        metadata.is_file()
            && metadata.len() > 0
            && metadata
                .modified()
                .is_ok_and(|timestamp| timestamp >= newest_dependency)
    })
}

/// Runs a command and returns an error if it cannot start or exits unsuccessfully.
pub fn run_command(mut command: Command) -> io::Result<()> {
    let invocation = format!("{command:?}");

    // `.status()` runs the command.
    let status = command.status().map_err(|error| {
        io::Error::new(error.kind(), format!("failed to run {invocation}: {error}"))
    })?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "{invocation} exited unsuccessfully: {status}"
        )))
    }
}

/// Creates a directory and any missing parent directories.
pub fn create_directory(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("failed to create directory {}: {error}", path.display()),
        )
    })
}

/// Verifies that a required path exists and is a directory.
pub fn require_directory(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("required directory {} is missing: {error}", path.display()),
        )
    })?;

    if metadata.is_dir() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "required directory {} is not a directory",
            path.display()
        )))
    }
}

/// Verifies that a required path exists and is a non-empty file.
pub fn require_nonempty_file(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("required file {} is missing: {error}", path.display()),
        )
    })?;

    if metadata.is_file() && metadata.len() > 0 {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "required file {} is not a non-empty file",
            path.display()
        )))
    }
}
