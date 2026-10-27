# lvertiz4.github.io

Personal portfolio: a landing page with project cards, one page per project
(with room for an embedded interactive demo), and an About page.

Built with [Leptos](https://leptos.dev) 0.8 as a **static site**. Every page is
rendered to HTML once, at build time, so it loads instantly and is readable by
search engines. The same code is also compiled to WebAssembly and *hydrates*
the pre-rendered HTML in the browser, so client-side navigation and future
interactive demos work. No server runs in production; GitHub Pages serves plain
files.

## Layout

```
Cargo.toml              crate config + [package.metadata.leptos] (cargo-leptos settings)
src/lib.rs              module tree + the wasm `hydrate` entry point
src/main.rs             ssr-only binary: pre-renders pages to dist/, or runs the dev server
src/app.rs              HTML shell + router (all routes are SsrMode::Static)
src/site.rs             your name, tagline, URLs
src/projects.rs         project data: a `static` slice of structs
src/pages/              one component per route (home, project, about, 404)
src/components/         header, footer, project card, tech tags
style/main.css          plain CSS -> dist/pkg/portfolio.css
demos/<slug>/           one small wasm crate per interactive demo (workspace members)
assets/                 copied verbatim into dist/ (.nojekyll, favicon)
.github/workflows/      build + deploy to GitHub Pages
dist/                   generated output (git-ignored)
```

Interactive demos live as separate wasm crates under `demos/`, picked up as
workspace members by `members = ["demos/*"]` in `Cargo.toml`. Each demo is a
thin wrapper that depends on the project's own repository as a git
dependency (for example `demos/saga-executor/` depends on `saga-core` from
<https://github.com/lvertiz4/saga-executor>), so the library stays in one
place and this repo only holds the browser glue.

## Prerequisites

```sh
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked    # or: cargo binstall cargo-leptos
```

cargo-leptos downloads matching `wasm-bindgen` and `wasm-opt` binaries on
first use.

## Local development

```sh
cargo leptos watch
```

Open <http://127.0.0.1:3000>. The dev server pre-renders every route on start,
serves `dist/`, and reloads the browser when files change.

## How the static build works

```sh
cargo leptos build --release          # 1. wasm + JS + CSS + assets -> dist/, ssr binary -> target/release/
./target/release/portfolio generate   # 2. render every static route -> dist/**/index.html
```

Step 2 is `main.rs`. It calls `leptos_axum::generate_route_list_with_ssg`,
which walks the `<Routes>` in `app.rs`, finds every route marked
`SsrMode::Static`, expands `:slug` using `StaticRoute::prerender_params`
(fed by `projects::slugs()`), renders each one, and writes the HTML under
`dist/`:

| Route                   | File                              |
| ----------------------- | --------------------------------- |
| `/`                     | `dist/index.html`                 |
| `/about/`               | `dist/about/index.html`           |
| `/projects/<slug>/`     | `dist/projects/<slug>/index.html` |

The trailing slash on route paths is what makes Leptos emit a directory with
an `index.html` (without it, `/about` would become `about.html`).

Serve the result like any static folder:

```sh
python3 -m http.server 8000 --directory dist
```

## Deploys

Pushing to `main` runs `.github/workflows/deploy.yml`, which installs Rust and
cargo-leptos, runs the two build steps above, and publishes `dist/` with
`actions/deploy-pages`.

One-time setup: in the repository on GitHub, open **Settings → Pages** and set
**Source** to **GitHub Actions**.

## Adding a project

1. Append a `Project { .. }` entry to `PROJECTS` in `src/projects.rs`. The
   `slug` becomes the URL (`/projects/<slug>/`).
2. That is all: the card appears on the landing page, and the static generator
   emits the new page because it enumerates `PROJECTS` for `:slug`.
3. When a demo exists, set `demo_url` and mount it into the
   `<section id="demo" data-demo="<slug>">` placeholder on the project page.

## Customising

- Name, tagline, GitHub and resume links: `src/site.rs`.
- Put your resume at `assets/resume.pdf`; it is served at `/resume.pdf`.
- Styles: `style/main.css`.
