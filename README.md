# daltoncole.com

Personal site, built with [Leptos](https://leptos.dev/). Every page is rendered to static HTML
at build time. Only the interactive tools in `src/islands/` run in the browser, as WebAssembly.

## Commands

Requires `cargo install cargo-leptos --version 0.3.11 --locked`.

```sh
cargo leptos watch                               # dev server with rebuilds, on 127.0.0.1:3000
cargo test --features ssr --no-default-features  # unit tests
cargo leptos build --release && ./target/release/daltoncole  # production build, in target/site
```

Pushing to `main` builds and deploys the site through `.github/workflows/deploy.yml`.

## Adding content

A Markdown file's path is its URL: `content/learn/rust/traits.md` is `/learn/rust/traits/`.
Each file starts with TOML front matter:

```
+++
title = "Traits"
+++
```

Parent Learn pages are assembled from their children, so a new topic only needs its own file.
Set `math = true` to use `$...$` and `$$...$$` on a page. Code blocks are highlighted at build
time. Files in `public/` are copied to the site root as they are.
