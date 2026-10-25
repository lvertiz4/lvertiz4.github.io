//! Project data.
//!
//! A `static` slice of plain structs. Because every field is `&'static str`,
//! the data is baked into both the wasm bundle and the pre-rendered HTML with
//! no parsing or allocation. To add a project, append an entry to `PROJECTS`;
//! the router enumerates this slice to decide which `/projects/<slug>/` pages
//! to generate.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Project {
    /// URL segment: `/projects/<slug>/`. Lowercase, hyphens only.
    pub slug: &'static str,
    pub title: &'static str,
    /// One sentence, shown on the card and used as the page's meta description.
    pub summary: &'static str,
    pub tech: &'static [&'static str],
    pub problem: &'static str,
    pub approach: &'static str,
    pub hard_thing: &'static str,
    pub results: &'static str,
    pub source_url: &'static str,
    /// `None` until a demo exists; the card then hides the Demo link.
    pub demo_url: Option<&'static str>,
}

impl Project {
    /// Site-relative URL of this project's page, with the trailing slash the
    /// static generator needs to emit `index.html`.
    pub fn url(&self) -> String {
        format!("/projects/{}/", self.slug)
    }
}

pub static PROJECTS: &[Project] = &[
    Project {
        slug: "saga-executor",
        title: "Saga Executor",
        summary: "Maintain data consistency across distributed systems without using blocking global database lock with Forward and Backwards Recovery. Correctness is enforced by Rust's fearless conccurrency.",
        tech: &["Rust", "WebAssembly", "Leptos"],
        problem: "Describe the problem this project solves and who has it.",
        approach: "Describe the overall design and the main trade-offs you made.",
        hard_thing:
            "Describe the one part that was genuinely difficult and how you got through it.",
        results: "Describe what the finished project achieves, with numbers if you have them.",
        source_url: "https://github.com/lvertiz4/saga-executor",
        demo_url: None,
    },
    Project {
        slug: "pub-sub-telemetry",
        title: "Pub-Sub Telemetry",
        summary: "Capturing metrics, logs, and distributed traces—across asynchronous Publish/Subscribe messaging systems. ",
        tech: &["Rust", "Tokio"],
        problem: "Problem statement goes here.",
        approach: "Approach goes here.",
        hard_thing: "The hard part goes here.",
        results: "Results go here.",
        source_url: "https://github.com/lvertiz4/pub-sub-telemetry",
        demo_url: None,
    },
    Project {
        slug: "fieldkit",
        title: "FieldKit",
        summary: "Avoid telementry fragility with swappable, open-source microcontroller blueprints paired with offline-first mobile apps.",
        tech: &["Rust", "Embedded"],
        problem: "Problem statement goes here.",
        approach: "Approach goes here.",
        hard_thing: "The hard part goes here.",
        results: "Results go here.",
        source_url: "https://github.com/lvertiz4/placeholder-three",
        demo_url: None,
    },
];

/// Look up a project by its URL slug.
pub fn find(slug: &str) -> Option<&'static Project> {
    PROJECTS.iter().find(|p| p.slug == slug)
}

/// Every slug, for the static generator to expand `/projects/:slug/`.
pub fn slugs() -> Vec<String> {
    PROJECTS.iter().map(|p| p.slug.to_string()).collect()
}
