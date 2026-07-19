use dioxus::prelude::*;

#[component]
pub fn Resources() -> Element {
    rsx! {
        document::Title { "Resources - Rust StuCo" }
        div { class: "max-w-prose mx-auto px-8 pt-16",
            h1 { class: "text-3xl font-bold italic text-primary mb-6 text-center",
                "Resources"
            }
            p { "Under construction" }
        }
    }
}

#[cfg(test)]
mod tests {
    use schemars::JsonSchema;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, JsonSchema)]
    struct ResourceFile {
        resources: Vec<ResourceEntry>,
    }

    #[derive(Debug, Deserialize, JsonSchema)]
    struct ResourceEntry {
        title: String,
        url: String,
        description: Option<String>,
        author: Option<String>,
        official: Option<bool>,
    }

    const RESOURCE_FILES: &[&str] = &[
        include_str!("../../resources/blog-posts.toml"),
        include_str!("../../resources/books.toml"),
        include_str!("../../resources/cheatsheets.toml"),
        include_str!("../../resources/interactive.toml"),
        include_str!("../../resources/meta.toml"),
        include_str!("../../resources/playlists.toml"),
    ];

    #[test]
    fn resource_files_are_valid() {
        let resources = RESOURCE_FILES
            .iter()
            .flat_map(|contents| {
                toml::from_str::<ResourceFile>(contents)
                    .expect("resource file should be valid TOML")
                    .resources
            })
            .collect::<Vec<_>>();

        assert!(!resources.is_empty());
        assert!(
            resources
                .iter()
                .any(|resource| resource.official.unwrap_or(false))
        );

        for resource in resources {
            assert!(!resource.title.trim().is_empty());
            assert!(resource.url.starts_with("https://"));

            if let Some(description) = resource.description {
                assert!(!description.trim().is_empty());
            }

            if let Some(author) = resource.author {
                assert!(!author.trim().is_empty());
            }
        }
    }

    #[test]
    fn resource_schema_is_current() {
        let schema = schemars::schema_for!(ResourceFile);
        let json = format!("{}\n", serde_json::to_string_pretty(&schema).unwrap());
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/resource.schema.json"
        );

        if std::env::var_os("UPDATE_SCHEMAS").is_some() {
            std::fs::write(path, json).unwrap();
        } else {
            assert_eq!(
                std::fs::read_to_string(path).unwrap(),
                json,
                "resource schema is stale; regenerate it with \
                 `UPDATE_SCHEMAS=1 cargo test schema_is_current`"
            );
        }
    }
}
