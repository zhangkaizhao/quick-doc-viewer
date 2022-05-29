mod app;
mod constants;
mod files;
mod query;
mod templates;

pub(crate) use files::{Files, MimeType, guess_mime_type, is_markdown, read_text_file_to_string, markdown};
pub(crate) use query::Page as PageQuery;
pub(crate) use templates::Templates;

pub use app::make_app;
