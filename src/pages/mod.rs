//! One module per route. Each page owns its `<Title>` and `<Meta>` tags.

mod about;
mod home;
mod not_found;
mod project;

pub use about::About;
pub use home::Home;
pub use not_found::NotFound;
pub use project::ProjectPage;
