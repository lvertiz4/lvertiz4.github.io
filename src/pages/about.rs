use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

use crate::site;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About"/>
        <Meta name="description" content=format!("About {}.", site::NAME)/>
        <Link rel="canonical" href=format!("{}/about/", site::URL)/>

        <article class="prose">
            <h1>"About"</h1>
            <p>
                "I build backend and distributed systems in Rust — async services with Axum and Tokio, an agentic graphRAG platform spanning vector search (Qdrant), graph databases (Neo4j), and local LLM inference (Ollama), with growing depth in embedded and low-level work down to the binary and assembly layers.

Before engineering, I spent a decade in policy andconflict analysis through performance auditing for government — work that was, at its core, failure analysis of complex systems under ambiguity: modeling how things break, tracing cause through incomplete evidence, and communicating risk to stakeholders with competing interests. I now apply that same lens to software: threat modeling, partial-failure reasoning, and documentation-first engineering (every project ships with an ARCHITECTURE.md and THREAT_MODEL.md).

I'm currently interested in backend infrastructure, software correctness, and building distributed systems where the failure modes catch risks rather than merely alert you to undefined behaviors."
            </p>
            <p>
                <a href=site::GITHUB_URL rel="me">"GitHub"</a>
                " · "
                <a href=site::RESUME_URL>"Resume (PDF)"</a>
            </p>
        </article>
    }
}
