use leptos::prelude::*;

use crate::site;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <a href=site::GITHUB_URL rel="me">"GitHub"</a>
            <span>" · Built with Leptos, pre-rendered to static HTML."</span>
        </footer>
    }
}
