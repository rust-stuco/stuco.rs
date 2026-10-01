use std::process::Command;

fn main() {
    if cfg!(unix) {
        Command::new("zip")
            .arg("-r")
            .arg("handin.zip")
            .arg("src/")
            .arg("Cargo.toml")
            .output()
            .expect("\nError: Unable to zip handin files. Either the zip executable is not installed on this computer, the zip binary is not on your PATH, or something went very wrong with zip. Please contact the staff for help!\n\n");
    }

    println!("cargo:rerun-if-changed=handin.zip");
}
