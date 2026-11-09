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

## Where things live

- `src/site/profile.rs`: Home, About, Work Experience, Résumé, and Contact.
- `src/site/pages.rs`: the Projects page and the list of pages on the site.
- `src/islands/`: the interactive tools.
- `public/`: copied to the site root as it is, including the PDFs and images.
