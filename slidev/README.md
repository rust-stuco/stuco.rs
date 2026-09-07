# Lecture slides with Slidev

All 14 published lecture decks use the pinned Slidev toolchain in this directory. Their Markdown
stays under `../lectures/NN_topic/topic.md`. Each deck loads the shared `stuco` addon from `runtime/`.
Slidev reads the lecture source files directly and refreshes the browser after edits.

## Develop a deck

From `slidev/`, pass the Markdown file after `--`:

```bash
npm ci
npm run dev -- ../lectures/01_introduction/introduction.md
```

Slidev serves the deck at `http://localhost:3030` and refreshes when its Markdown changes. Presenter
mode, notes, overview, drawings, and the theme toggle are available from the presentation controls.

## Build and export

```bash
npm run build -- ../lectures/01_introduction/introduction.md \
  --base /lectures/01_introduction/deck/ --router-mode hash \
  --out ../../slidev/dist/01_introduction/deck
npm run export:light -- ../lectures/01_introduction/introduction.md --output introduction-light.pdf
npm run export:dark -- ../lectures/01_introduction/introduction.md --output introduction-dark.pdf
```

These commands write the deck to `slidev/dist/01_introduction/deck/` and the PDFs to `slidev/`.
Slidev resolves `--out` relative to the lecture directory and `--output` relative to the current
directory. `npm ci` installs the Chromium browser used for PDF export. To use another browser,
add `--executable-path /path/to/browser` to the export command.

The full build runs these commands for every published lecture, one lecture at a time. After
`dx build --release`, it writes the decks and PDFs into the final site at
`target/dx/stuco-rs/release/web/public/lectures/`. See [CONTRIBUTING.md](../CONTRIBUTING.md#full-build)
for setup and the full build command.

The static decks use hash routing, so deep links survive reloads without server rewrites. They are
built after the website because the Dioxus asset pipeline cannot preserve Slidev's JavaScript module
graph. Local builds and CI use the same script to produce the complete site.

The sources under `lectures/review.md` and `lectures/graveyard/` are historical material, not
published decks, and are not supported by this toolchain.
