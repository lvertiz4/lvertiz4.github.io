use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

use crate::components::ProjectCard;
use crate::projects::PROJECTS;
use crate::site;

/// Landing page: the project grid.
///
/// `<Title>` and `<Meta>` here are hoisted into `<head>` by leptos_meta, so
/// the pre-rendered HTML carries them for crawlers and link previews.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Projects"/>
        <Meta name="description" content=site::DESCRIPTION/>
        <Meta property="og:title" content=site::NAME/>
        <Meta property="og:description" content=site::DESCRIPTION/>
        <Link rel="canonical" href=format!("{}/", site::URL)/>

        <section class="page-intro">
            <h1>"Projects"</h1>
            <p>"Selected work. Each page covers the problem, the approach, the hard part and the results."</p>
        </section>
        <section class="project-grid" aria-label="Projects">
            {PROJECTS.iter().map(|p| view! { <ProjectCard project=p/> }).collect_view()}
        </section>
    }
}
