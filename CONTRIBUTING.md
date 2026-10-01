# Contributing

## Website

Install Rust with `rustup`. Then prepare the website tools:

```bash
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.2 --locked
```

From the repository root, start the website:

```bash
dx serve
```

`dx serve`, `dx build`, and `cargo test` build only the website. The full build includes the syllabus,
lecture decks, PDFs, and homework downloads.

## Lecture slides

Install Node.js 22 and npm. Then install the Slidev dependencies and start a lecture:

```bash
cd slidev
npm ci
npm run dev -- 09_ownership_p2
```

Slidev serves the deck at `http://localhost:3030` and refreshes after Markdown changes.
See [the Slidev README](slidev/README.md) for individual deck builds and PDF exports.

Blocks that show cargo, rustc, or test output use the `ansi` language, so they keep cargo's colors. Capture real output instead of writing escape codes by hand:

```bash
TERM=ansi CARGO_TERM_HYPERLINKS=false cargo +stable build --color=always > out.ansi 2>&1
TERM=ansi CARGO_TERM_HYPERLINKS=false cargo +stable test --color=always -- --color=always > out.ansi 2>&1
```

`TERM=ansi` makes the test harness reset colors with codes Slidev renders, and `CARGO_TERM_HYPERLINKS=false` removes link escapes that would show up as text. Put the output in a code block tagged `ansi`. You can delete lines and shorten paths, but keep the escape codes intact.

## Full build

Complete the website and Slidev setup first. Also install Python 3.11 or later,
[Typst](https://typst.app/open-source/#download), and Chrome or Chromium.
If the browser is not in a standard location, set `STUCO_SLIDEV_CHROME` to its executable path.

From the repository root, run the same build command as CI:

```bash
python3 scripts/build.py
```

The script builds the website, then generates the syllabus, lecture decks, PDFs, and homework
downloads. It checks the generated files before it succeeds. The complete site is in
`target/dx/stuco-rs/release/web/public/`.

To preview the complete site locally, run:

```bash
npx wrangler@4 dev --local
```

Wrangler uses the output directory in `wrangler.jsonc` and prints the local URL.
After source changes, run the full build again to update the preview.

## Tests

From the repository root, run the website tests:

```bash
cargo test --locked
```

To test the build script, run:

```bash
python3 -m unittest discover -s scripts -p 'test_*.py'
```
