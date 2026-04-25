mod blog;
mod experience;
mod project;

use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum ContentType {
    Blog,
    Experience,
    Project,
}

pub trait Content {
    fn content_type(&self) -> ContentType;
}

pub trait Promptable: Sized {
    fn prompt() -> Self;
}

pub use blog::Blog;
pub use experience::Experience;
pub use project::Project;