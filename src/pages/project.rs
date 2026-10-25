use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use leptos_router::hooks::use_params_map;

use crate::components::TechTags;
use crate::pages::NotFound;
use crate::projects::{self, Project};
use crate::site;

/// `/projects/:slug/`. Reads the slug from the URL and looks it up in the
/// static project data. The slugs were enumerated at build time, so on the
/// deployed site this always finds a project; the `None` branch covers the
/// dev server and client-side navigation to a bad slug.
#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params_map();
    // `read()` is reactive: if the user navigates between project pages
    // client-side, this closure re-runs and the view updates in place.
    let project = move || {
        let params = params.read();
        params.get("slug").and_then(|slug| projects::find(&slug))
    };

    move || match project() {
        Some(p) => view! { <ProjectDetail project=p/> }.into_any(),
        None => view! { <NotFound/> }.into_any(),
    }
}

#[component]
fn ProjectDetail(project: &'static Project) -> impl IntoView {
    view! {
        <Title text=project.title/>
        <Meta name="description" content=project.summary/>
        <Meta property="og:title" content=project.title/>
        <Meta property="og:description" content=project.summary/>
        <Link rel="canonical" href=format!("{}{}", site::URL, project.url())/>

        <article class="project">
            <header class="project__header">
                <h1>{project.title}</h1>
                <p class="project__summary">{project.summary}</p>
                <TechTags tags=project.tech/>
                <p class="project__links">
                    {project.demo_url.map(|href| view! { <a href=href>"Live demo"</a> })}
                    <a href=project.source_url rel="noopener">"Source on GitHub"</a>
                </p>
            </header>

            // Empty mount point for a future wasm demo crate. The slug lets a
            // loader script pick the right bundle later.
            <section id="demo" class="demo-mount" data-demo=project.slug aria-label="Interactive demo">
                <p class="demo-mount__placeholder">"Interactive demo coming soon."</p>
            </section>

            <Section heading="Problem" body=project.problem/>
            <Section heading="Approach" body=project.approach/>
            <Section heading="One hard thing" body=project.hard_thing/>
            <Section heading="Results" body=project.results/>
        </article>
    }
}

/// A heading plus a paragraph; keeps the page body declarative.
#[component]
fn Section(heading: &'static str, body: &'static str) -> impl IntoView {
    view! {
        <section class="project__section">
            <h2>{heading}</h2>
            <p>{body}</p>
        </section>
    }
}
