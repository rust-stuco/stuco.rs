# Contributing

## Setup

### Syllabus

This project uses the `typst` CLI to compile the [syllabus source file](src/syllabus.typ) into a PDF located at `public/syllabus.pdf` during the build process. You need to have `typst` installed on your system for this to work.

Download and install `typst` from the [official website](https://typst.app/open-source/#download)!

### Lecture slides

The build renders every lecture as an interactive [Slidev](slidev/README.md) deck and as light and
dark PDFs. Install Node.js, npm, and Chrome or Chromium. The build installs the pinned JavaScript
dependencies from `slidev/package-lock.json`; set `STUCO_SLIDEV_CHROME` if it cannot find your
browser on `PATH` or in a standard installation directory.

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

The first run takes longer because it builds the syllabus, homework handouts, website, and all
lecture formats.

### Lecture decks

`dx serve` serves each deck at `/lectures/NN_topic/deck/`, the same path used in deployments. The
schedule links the interactive deck and both PDF variants.

When editing one deck, run `npm run dev -- NN_topic` from `slidev/` for faster feedback. For example:

```bash
cd slidev
npm run dev -- 09_ownership_p2
```

See [the Slidev README](slidev/README.md) for build, export, and output details.
