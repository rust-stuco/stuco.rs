# Contributing

## Setup

### Syllabus

This project uses the `typst` CLI to compile the [syllabus source file](src/syllabus.typ) into a PDF located at `public/syllabus.pdf` during the build process. You need to have `typst` installed on your system for this to work.

Download and install `typst` from the [official website](https://typst.app/open-source/#download)!

### Lecture slides

The build renders every lecture deck to PDF with the `marp` CLI, so you need it installed even if you
are only working on the website:

```bash
npm install -g @marp-team/marp-cli
```

Week one is also being rewritten in [Slidev](slidev/README.md) as an eventual replacement for marp.
That deck is rendered into `public/slidev/week01/` by the build, so you need `node` and `npm`
installed as well. The build installs the pinned Slidev toolchain into `slidev/node_modules/` itself.

### Website

You need to install `dioxus-cli` to build and run the website. Pin the same version CI uses, since a
mismatch can fail to build the crate:

```bash
cargo install dioxus-cli --version 0.7.2
```

Alternatively, you can use `cargo binstall` to install a pre-built binary (faster):

```bash
cargo install cargo-binstall
cargo binstall dioxus-cli@0.7.2
```

## Development

Run the following command in the root of your project:

```bash
dx serve
```

The first time you run this, it will take longer since it needs to build the `public/` directory from `homeworks/`, `lectures/`, `slidev/`, and `src/syllabus.typ`.
