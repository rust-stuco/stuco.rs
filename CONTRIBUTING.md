# Contributing

## Setup

### Syllabus

This project uses the `typst` CLI to compile the [syllabus source file](src/syllabus.typ) into a PDF located at `public/syllabus.pdf` during the build process. You need to have `typst` installed on your system for this to work.

Download and install `typst` from the [official website](https://typst.app/open-source/#download)!

### Lecture slides

The build renders the lecture decks from week two onwards to PDF with the `marp` CLI, so you need it
installed even if you are only working on the website:

```bash
npm install -g @marp-team/marp-cli
```

Week one is rendered by [Slidev](slidev/README.md) instead, as the eventual replacement for marp. The
build writes both its slide PDFs and a browsable copy of the deck at `public/slidev/week01/`, so you
also need `node` and `npm`; the pinned Slidev toolchain is installed into `slidev/node_modules/` by
the build itself.

Both renderers draw their PDFs in a browser, so you need Chrome or Chromium installed. Set
`STUCO_SLIDEV_CHROME` if yours is not on `PATH` under a usual name.

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

### The week one deck

`dx` runs every JavaScript file under `public/` through its own asset pipeline, and re-bundling
Slidev's chunks leaves the deck a blank page. The build therefore writes the deck to
`target/slidev/week01/`, and the deploy workflow overlays that onto the bundle it uploads. To get the
same thing locally, copy it into the served bundle once after your first build:

```bash
mkdir -p target/dx/stuco-rs/debug/web/public/slidev
cp -r target/slidev/week01 target/dx/stuco-rs/debug/web/public/slidev/
```

The copy survives later rebuilds, since `dx` only syncs what is inside `public/`. The slide PDFs need
none of this — `dx` copies binary files through untouched.

When working on the deck itself, `cd slidev && npm run dev` is faster and reloads on save.
