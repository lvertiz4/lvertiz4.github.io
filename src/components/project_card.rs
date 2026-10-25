use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::TechTags;
use crate::projects::Project;

/// One tile in the landing-page grid.
#[component]
pub fn ProjectCard(project: &'static Project) -> impl IntoView {
    view! {
        <article class="project-card">
            <h3 class="project-card__title">
                <A href=project.url()>{project.title}</A>
            </h3>
            <p class="project-card__summary">{project.summary}</p>
            <TechTags tags=project.tech/>
            <p class="project-card__links">
                // Option<impl IntoView> renders nothing for None.
                {project.demo_url.map(|href| view! { <a href=href>"Demo"</a> })}
                <a href=project.source_url rel="noopener">"Source"</a>
            </p>
        </article>
    }
}
