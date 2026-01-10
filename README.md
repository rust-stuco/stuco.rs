# 98-008 Website

## Setup

This project uses the `typst` CLI to compile the [syllabus source file](src/syllabus.typ) into a PDF located at `public/syllabus.pdf` during the build process. You need to have `typst` installed on your system for this to work. On macOS, you can install it using Homebrew:

```bash
brew install typst
```

You also need to install `dioxus-cli` to run the development server:

```bash
cargo install cargo-binstall # if you don't already have binstall
cargo binstall dioxus-cli
```

## Development

Run the following command in the root of your project:

```bash
dx serve
```

The first time you run this, it will take longer since it needs to build the `public/` directory from `homeworks/`, `lectures/`, and `src/syllabus.typ`.
