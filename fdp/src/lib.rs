
pub mod action;
pub mod state;
pub mod types;

pub fn read_source_path(path: &str) -> Result<types::Source, String> {
    types::Source::new(path).ok_or("Directory does not exist".into())
}
