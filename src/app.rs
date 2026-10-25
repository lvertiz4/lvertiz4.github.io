//! The HTML shell and the router. Every page starts here.

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
    static_routes::{StaticParamsMap, StaticRoute},
    SsrMode,
};

use crate::components::{Footer, Header};
use crate::pages::{About, Home, NotFound, ProjectPage};
use crate::{projects, site};

/// The full HTML document. Only the server/generator renders this; the wasm
/// bundle hydrates `<body>` and leaves `<head>` alone.
///
/// - `AutoReload`: injects the live-reload script only under `cargo leptos watch`.
/// - `HydrationScripts`: `<link rel=modulepreload>` + the script that loads the wasm.
/// - `MetaTags`: where `<Title>`/`<Meta>` set anywhere in the app are collected.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/favicon.svg" type="image/svg+xml"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// Root component: shared chrome plus the route table.
#[component]
pub fn App() -> impl IntoView {
    // Lets <Title>, <Meta> and <Stylesheet> below (and in pages) reach <head>.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        // Pages set a short <Title text=.../>; this formatter appends the site name.
        <Title formatter=|text| format!("{text} · {}", site::NAME)/>
        <Meta name="color-scheme" content="light dark"/>

        <Router>
            <Header/>
            <main id="main">
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home ssr=prerender()/>
                    <Route path=path!("/about/") view=About ssr=prerender()/>
                    // Trailing slashes matter: "/about/" is written to about/index.html,
                    // which GitHub Pages serves at /about/. Without it we'd get about.html.
                    <Route
                        path=path!("/projects/:slug/")
                        view=ProjectPage
                        ssr=SsrMode::Static(
                            StaticRoute::new().prerender_params(|| async { project_params() }),
                        )
                    />
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}

/// Routes with no params: render once at build time.
fn prerender() -> SsrMode {
    SsrMode::Static(StaticRoute::new())
}

/// Tells the generator which values `:slug` takes, so every project page is emitted.
fn project_params() -> StaticParamsMap {
    let mut params = StaticParamsMap::new();
    params.insert("slug", projects::slugs());
    params
}
