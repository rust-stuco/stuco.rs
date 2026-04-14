# Contributing

## Setup

### Syllabus

This project uses the `typst` CLI to compile the [syllabus source file](src/syllabus.typ) into a PDF located at `public/syllabus.pdf` during the build process. You need to have `typst` installed on your system for this to work.

Download and install `typst` from the [official website](https://typst.app/open-source/#download)!

### Website

You need to install `dioxus-cli` to build and run the website:

```bash
cargo install dioxus-cli
```

Alternatively, you can use `cargo binstall` to install a pre-built binary (faster):

```bash
cargo install cargo-binstall
cargo binstall dioxus-cli
```

## Development

Run the following command in the root of your project:

```bash
dx serve
```

The first time you run this, it will take longer since it needs to build the `public/` directory from `homeworks/`, `lectures/`, and `src/syllabus.typ`.
