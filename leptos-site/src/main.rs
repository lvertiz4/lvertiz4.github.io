use leptos::{logging::Log, prelude::*};

#[component]
fn App() -> impl IntoView {
    view!{
        <h1>"Hello from Somi's crate!"</h1>
        <button on:click=move |_| log!("Button Clicked with paw!")>"Click me"</button>
    }
}

fn main() {
    leptos::mount_to_body(App);
}