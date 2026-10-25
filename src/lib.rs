//! Portfolio site library.
//!
//! This crate is compiled twice by cargo-leptos:
//! - as a `cdylib` with the `hydrate` feature -> the wasm bundle loaded by the browser;
//! - as an `rlib` with the `ssr` feature -> linked into `main.rs`, which renders
//!   every page to HTML at build time.
//!
//! The modules below are shared by both builds.

pub mod app;
pub mod components;
pub mod pages;
pub mod projects;
pub mod site;

/// Entry point for the wasm bundle.
///
/// `HydrationScripts` in `app::shell` emits a `<script>` that loads the wasm
/// and calls this function. `hydrate_body` walks the HTML that was pre-rendered
/// at build time and attaches the reactive system to it instead of re-creating
/// the DOM, which is what makes the page interactive without a flash.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    // Route panics to console.error with the Rust message and location.
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
