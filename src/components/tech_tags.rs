use leptos::prelude::*;

/// A row of small technology labels.
///
/// The data is `'static`, so a plain iterator into a `Vec<_>` of views is
/// enough; `<For>` is only needed when a list changes at runtime.
#[component]
pub fn TechTags(tags: &'static [&'static str]) -> impl IntoView {
    view! {
        <ul class="tech-tags" aria-label="Technologies">
            {tags.iter().map(|t| view! { <li class="tech-tag">{*t}</li> }).collect_view()}
        </ul>
    }
}
