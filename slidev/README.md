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

Each command gets its own workspace under `.slidev-work/`, so a site build
triggered by `dx serve` cannot delete the workspace that `npm run dev` is
serving from.

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
npm run export:light
npm run export:dark
```

Generated files are written to `dist/`, which is intentionally ignored. The
static site goes in `dist/site/` and uses `/slidev/week01/` as its deployment
base.

Export renders in a browser. Slidev drives it through Playwright, which would
otherwise download its own copy, so the commands pass `--executable-path` for the
first Chrome or Chromium found on `PATH` — the same assumption the marp config
makes. Set `STUCO_SLIDEV_CHROME` to override the choice.

## Deployment

`build.rs` runs `npm ci`, `npm run build`, and both export commands here, setting
`STUCO_SLIDEV_SITE_OUTPUT` and `STUCO_SLIDEV_PDF_OUTPUT` so the output lands in
the tree rather than in `dist/`. Week one therefore ships two ways, in production
and in every pull request preview:

- the deck itself at `/slidev/week01/`, linked from the resources page
- `introduction-light.pdf` and `introduction-dark.pdf` under
  `/lectures/01_introduction/`, which is where the schedule page links slides

The PDFs go directly into `public/`, but the deck goes to `target/slidev/week01/`
and is overlaid onto the bundle by the deploy workflow. It cannot live in
`public/`: `dx` runs every JavaScript file there through its asset pipeline, and
re-bundles each of Slidev's chunks into a standalone copy of the entire deck —
every chunk came out at roughly 811 kB, module identity was lost, and only one
slide ever mounted. Binary files like the PDFs are copied through untouched.

The build then copies the deck into the bundle `dx` serves, so `dx serve` offers
the deployed paths without a manual step. That copy targets a layout `dx` owns,
so it is best-effort; the deploy workflow overlays the deck explicitly, and that
is what production relies on.

Because Slidev owns those PDFs, `build/lectures.rs` no longer renders week one
with marp. Marp would otherwise print the per-slide `layout:` and `class:` blocks
as slide text and lose the images, and it can no longer produce a dark variant
now that the `class: invert` directive it keyed on is gone.

Slidev routes slides client-side, so the deployed build uses `--router-mode hash`
and slide links look like `/slidev/week01/#/12`. Hash routes never reach the
server, which means deep links survive a reload without the host having to
rewrite unknown paths back to the deck. `npm run dev` keeps the default history
routing, since only Slidev's own server has to satisfy it.

For the same reason the build deletes the `_redirects` file Slidev generates.
That rule points its own wildcard back at `index.html`, which Cloudflare rejects
as an infinite loop, and hash routing leaves nothing that needs it.

The image classes in `runtime/style.css` center their picture with `top` and
`bottom` insets rather than a `transform`. A transformed element is rasterized for
printing, and Chrome tiles that layer into the exported PDF, which cut the images
into strips with seams through them.
