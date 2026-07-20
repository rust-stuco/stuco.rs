# Week 1 Slidev experiment

This directory contains the package and commands for the week-one Slidev
experiment. The deck remains at its existing path:
`../lectures/01_introduction/introduction.md`.

Slidev normally discovers CSS, Vue components, Vite configuration, and its
generated cache beside the deck entry point. To keep those files out of
`lectures/`, the commands below create an ignored temporary workspace under
this directory. The workspace links to the existing deck and lecture images,
copies in the files from `runtime/`, and is refreshed on each run. It is
normally removed when Slidev exits; if the process is forcibly killed, the
ignored workspace still remains contained inside `slidev/`.

The runtime Markdown hook also wraps lists written with `*` in Slidev click
groups. Lecture Markdown can therefore keep using ordinary bullet lists while
retaining incremental reveals, including parent-then-child ordering for nested
lists, in presentation mode.

## Run the presentation

```bash
npm install
npm run dev
```

Slidev serves the presentation at `http://localhost:3030`. Presenter mode,
overview, drawing, and the browser exporter are available from the navigation
controls in the lower-left corner.

## Build and export

```bash
npm run build
npx playwright install chromium
npm run export:light
npm run export:dark
```

Generated files are written to `dist/`, which is intentionally ignored. The
static site goes in `dist/site/` and uses `/slidev/week01/` as its deployment
base. Chromium is only needed for PDF export. This experiment is not yet
connected to the course website build pipeline.
