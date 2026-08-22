# Lecture slides with Slidev

All 14 published lecture decks use the pinned Slidev toolchain in this directory. Their Markdown
stays under `../lectures/NN_topic/topic.md`; shared runtime files stay under `runtime/`.

The commands create an ignored workspace for the selected lecture under `.slidev-work/`. They copy
the lecture directory and runtime into it, then link the shared images directory. Keeping Slidev's
generated files here prevents it from adding configuration and cache files beside the source decks.
On Windows, the shared image directory uses a junction and the lecture files are copied, so the
workflow does not require Developer Mode or administrator privileges.

## Develop a deck

Pass the lecture directory after `--`:

```bash
npm ci
npm run dev -- 01_introduction
```

Slidev serves the deck at `http://localhost:3030` and refreshes when its Markdown changes. Presenter
mode, notes, overview, drawings, and the theme toggle are available from the presentation controls.

## Build and export

```bash
npm run build -- 01_introduction
npm run export:light -- 01_introduction
npm run export:dark -- 01_introduction
```

By default, generated files go to `dist/01_introduction/`: the interactive deck is under `deck/`,
and the two PDFs sit beside it. Set `STUCO_SLIDEV_SITE_OUTPUT`, `STUCO_SLIDEV_PDF_OUTPUT`, or
`STUCO_SLIDEV_CHROME` to override those locations or the browser used for PDF export.

The Rust build script runs these commands for every published lecture. It installs the exact npm
dependencies recorded in `package-lock.json`, renders at most two lectures at once, writes PDFs to
`public/lectures/`, and stages interactive decks for `/lectures/NN_topic/deck/`.

The static decks use hash routing, so deep links survive reloads without server rewrites. They are
built outside `public/` because the Dioxus asset pipeline cannot preserve Slidev's JavaScript module
graph. The deployment workflow overlays the completed sites onto the Dioxus bundle after its build.

The sources under `lectures/review.md` and `lectures/graveyard/` are historical material, not
published decks, and are not supported by this toolchain.
