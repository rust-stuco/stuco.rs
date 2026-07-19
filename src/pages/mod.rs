use crate::navbar::Navbar;
use dioxus::prelude::*;

mod about;
mod home;
mod resources;
mod schedule;
mod semesters;

use about::About;
use home::Home;
use resources::Resources;
use schedule::Schedule;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/resources")]
        Resources {},
        #[route("/schedule")]
        Schedule {},
}

#[cfg(test)]
fn assert_schema_is_current<T: schemars::JsonSchema>(relative_path: &str) {
    use std::path::Path;

    let schema = schemars::generate::SchemaSettings::draft07()
        .into_generator()
        .into_root_schema_for::<T>();
    let generated = format!(
        "{}\n",
        serde_json::to_string_pretty(&schema).expect("schema should serialize as JSON")
    );
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);

    if std::env::var_os("UPDATE_SCHEMAS").is_some() {
        std::fs::write(&path, generated)
            .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
        return;
    }

    let checked_in = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    assert_eq!(
        checked_in,
        generated,
        "{} is stale; regenerate schemas with \
         `UPDATE_SCHEMAS=1 cargo test schema_is_current`",
        path.display()
    );
}
