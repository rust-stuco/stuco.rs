use dioxus::prelude::*;

static VSCODE_FORMAT: Asset = asset!("/assets/vscode-format-on-save.png");
static VSCODE_CLIPPY: Asset = asset!("/assets/vscode-rust-analyzer-clippy.png");

#[component]
pub(super) fn Setup() -> Element {
    rsx! {
        document::Title { "Setup - Rust StuCo" }
        div { class: "max-w-4xl mx-auto px-8 space-y-12 pb-16",
            h1 { class: "text-6xl font-bold text-primary mb-12 text-center", "Setup" }

            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Rust Installation" }
                p {
                    "To install Rust, we recommend you go through the official installation process with "
                    code { "rustup" }
                    ", Rust’s toolchain manager. Go to the "
                    a {
                        class: "text-secondary hover:underline",
                        href: "https://rust-lang.org/tools/install/",
                        target: "_blank",
                        "official website"
                    }
                    " and follow the instructions there for your OS, and make sure to choose all the default options. If you run into any issues, please post on Discord, as other people will likely have similar problems!"
                }
            }

            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Rust Analyzer" }
                p {
                    "If you use Visual Studio Code, install the "
                    a {
                        class: "text-secondary hover:underline",
                        href: "https://code.visualstudio.com/docs/languages/rust#_2-install-the-rust-analyzer-extension",
                        target: "_blank",
                        "rust-analyzer extension"
                    }
                    "!"
                }
                p {
                    "If you don't use Visual Studio Code, rust-analyzer is just a standard LSP server, so you can manually install it with "
                    code { "rustup component add rust-analyzer" }
                    ". Setup guides for just about any editor can easily be found online!"
                }
            }

            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Format and Lint Setup (Visual Studio Code)" }
                p {
                    "If you use Visual Studio Code, we recommend creating a folder and enabling \"Format on Save\" in your Workspace Settings. This will let rust-analyzer format your code whenever you save it, which saves you from having to run "
                    code { "cargo fmt" }
                    " every submission."
                }
                img { src: "{VSCODE_FORMAT}" }
                p {
                    "Next, change your rust analyzer check command to "
                    code { "clippy" }
                    ", which does everything that "
                    code { "check" }
                    " does while also highlighting linting issues directly in Visual Studio Code."
                }
                img { src: "{VSCODE_CLIPPY}" }
            }

            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Helpful Commands" }
                p {
                    "You may find these commands helpful while working on the labs! Make sure to run these in your lab's directory (where the "
                    code { "Cargo.toml" }
                    " file is)."
                }
                ul { class: "list-disc list-inside",
                    li {
                        "To test your code: "
                        code { "cargo test" }
                    }
                    li {
                        "To test your code with speed optimizations: "
                        code { "cargo test --release" }
                    }
                    li {
                        "To test your code with print output shown: "
                        code { "cargo test -- --nocapture" }
                    }
                }
                p {
                    "If you are having trouble with the autograder rejecting code style, run these two commands:"
                }
                ul { class: "list-disc list-inside",
                    li {
                        "To format your code: "
                        code { "cargo fmt" }
                    }
                    li {
                        "To fix linting errors in your code: "
                        code { "cargo clippy --fix --allow-no-vcs" }
                    }
                }
                p {
                    "You should never have to format your code by hand \u{2014} Cargo can do that for you with the format command mentioned above! And if you set up Visual Studio Code correctly, the two commands should run automatically."
                }
            }

            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Homework Submissions" }
                p {
                    "If you're on a unix system e.g. MacOS, upload "
                    code { "handin.zip" }
                    " to the Gradescope assignment (this is automatically generated for you whenever you build, run, check, test, etc). Otherwise, zip the "
                    code { "src" }
                    " folder and upload that."
                }
            }

            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold", "Grading" }
                p {
                    "For most labs, the autograder score will be your actual score, but "
                    strong { "you can resubmit as many times as you want" }
                    "! Your most recent submission's score will be used as your grade."
                }
                p {
                    "If you receive a 0 for your submission but your "
                    code { "cargo test" }
                    " still passes, that likely means you did not format and lint your code by running the commands above. Reach out to us if you have any questions!"
                }
            }
        }
    }
}
