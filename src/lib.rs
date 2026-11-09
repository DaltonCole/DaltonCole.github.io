//! Dalton Cole's personal site.
//!
//! Every page is rendered to static HTML at build time by the [`site`] module. The only
//! Rust that reaches the browser is [`islands`], the interactive tools.

// Large pages build deeply nested view types.
#![recursion_limit = "512"]

pub mod islands;

#[cfg(feature = "ssr")]
pub mod site;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
