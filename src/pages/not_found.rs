use leptos::prelude::*;
use leptos_meta::Title;

/// Fallback for unknown routes, including `/projects/<unknown-slug>/` in dev.
///
/// On the static host there is no server to return 404s; GitHub Pages serves
/// its own 404 page for paths that have no file. This component matters for
/// the dev server and for client-side navigation to a bad URL.
#[component]
pub fn NotFound() -> impl IntoView {
    // When rendering on the server, set the real HTTP status.
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        if let Some(resp) = use_context::<ResponseOptions>() {
            resp.set_status(axum::http::StatusCode::NOT_FOUND);
        }
    }

    view! {
        <Title text="Not found"/>
        <article class="prose">
            <h1>"Page not found"</h1>
            <p><a href="/">"Back to projects"</a></p>
        </article>
    }
}
