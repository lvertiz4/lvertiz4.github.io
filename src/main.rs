//! Native binary, only built with the `ssr` feature.
//!
//! Two jobs:
//! 1. `portfolio generate`  -> pre-render every static route to `<site-root>/**/index.html` and exit.
//!    This is the step CI runs after `cargo leptos build --release`.
//! 2. `portfolio`           -> do the same, then serve `<site-root>` on `site-addr`.
//!    `cargo leptos watch` uses this mode for local development with auto-reload.
//!
//! No server runs in production: GitHub Pages serves the files written by step 1.

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list_with_ssg, LeptosRoutes};
    use portfolio::app::shell;

    // Reads [package.metadata.leptos] from Cargo.toml; LEPTOS_* env vars
    // (set by `cargo leptos watch`) override individual values.
    let conf = get_configuration(Some("Cargo.toml"))
        .expect("run from the repository root so Cargo.toml can be found");
    let leptos_options = conf.leptos_options;

    // Walk the <Routes> tree in app.rs. Returns the axum route list plus a
    // generator for every route marked SsrMode::Static, with `:slug` params
    // expanded via StaticRoute::prerender_params.
    let (routes, static_routes) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    // Render each static route and write it under <site-root>.
    // "/" -> index.html, "/about/" -> about/index.html, etc.
    static_routes.generate(&leptos_options).await;
    println!("Static pages written to {}/", leptos_options.site_root);

    if std::env::args().nth(1).as_deref() == Some("generate") {
        return;
    }

    // Development server: serves the generated files and hot-reloads under cargo-leptos.
    let addr = leptos_options.site_addr;
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    println!("Dev server listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind dev server address");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("dev server exited with an error");
}

/// The wasm build has no `main`; see `lib::hydrate` instead.
#[cfg(not(feature = "ssr"))]
pub fn main() {}
