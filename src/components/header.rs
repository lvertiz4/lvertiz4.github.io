use leptos::prelude::*;
use leptos_router::components::A;

use crate::site;

/// Site header: name, tagline and primary navigation.
///
/// `<A>` is the router's anchor. It renders a normal `<a href>` (so the
/// pre-rendered HTML works without JS) and, once hydrated, navigates
/// client-side and sets `aria-current="page"` on the active link.
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="site-header__identity">
                <A href="/" attr:class="site-header__name">{site::NAME}</A>
                <p class="site-header__tagline">{site::TAGLINE}</p>
            </div>
            <nav class="site-nav" aria-label="Primary">
                <A href="/">"Projects"</A>
                <A href="/about/">"About"</A>
                // Plain <a>: a PDF is not a route, so skip client-side navigation.
                <a href=site::RESUME_URL target="_blank" rel="noopener">"Resume"</a>
            </nav>
        </header>
    }
}
