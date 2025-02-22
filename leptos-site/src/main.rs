use leptos::{logging::log, prelude::*};

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Hello from Somi's crate!"</h1>
        <button on:click=move |_| log!("Button clicked with paw!")>"Click me!"</button>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}