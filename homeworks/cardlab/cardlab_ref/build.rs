use std::process::Command;

fn main() {
    if cfg!(unix) {
        Command::new("zip")
            .arg("-r")
            .arg("handin.zip")
            .arg("src/")
            .output()
            .expect("\nError: Unable to zip handin files. Either zip is not installed on this computer, or something went very wrong with zip. If zip is already installed , please contact the staff for help!\n\n");
    }

    println!("cargo:rerun-if-changed=handin.zip");
}
