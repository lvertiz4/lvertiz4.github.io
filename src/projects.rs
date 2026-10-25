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
        problem: "Distributed systems have no global transaction. Provisioning a VM touches a database, a hypervisor, a network controller and a storage service, and any of them can fail or time out after the others succeeded.",
        approach: "The saga pattern turns 'either all or nothing' into 'every forward step has a recorded undo,' which is the strongest guarantee available when you cannot lock across machines.",
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
        problem: "Embedded software in drones, vehicles and sensors on radio links thathcannot use TCP's 'deliver everything in order' model: a retransmit of a position from two seconds ago blocks the fresh one behind it. But a command cannot be fire-and-forget either. So the problem is giving each message class the right guarantee with bounded memory, and proving the guarantees hold under loss.",
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
        problem: "Python's GIL and asyncio make a concurrent, cancellable, memory-bounded runtime painful. Rust cores should move data across FFI clearly, releasing the GIL so Rust threads actually run in parallel, and packaging fieldkit so it installs cleanly.",
        approach: "A Tokio-based agent runtime written in Rust and exposed to Python as a package through PyO3 and maturin. The runtime owns the concurrency (tool calls, retries, timeouts, cancellation) and Python owns the parts Python is good at: model calls, prompts, data science glue. The Python package is pip install-able and looks native to a Python user.",
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
