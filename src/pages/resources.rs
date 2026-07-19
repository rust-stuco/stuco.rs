use dioxus::prelude::*;

#[component]
pub(super) fn Resources() -> Element {
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
    use crate::pages::assert_schema_is_current;
    use schemars::JsonSchema;
    use serde::Deserialize;

    #[derive(Deserialize, JsonSchema)]
    struct ResourceFile {
        resources: Vec<ResourceEntry>,
    }

    #[derive(Deserialize, JsonSchema)]
    struct ResourceEntry {
        title: String,
        url: String,
        description: Option<String>,
        author: Option<String>,
        #[serde(default)]
        official: bool,
    }

    #[derive(Clone, Copy)]
    struct ResourceSource {
        path: &'static str,
        contents: &'static str,
    }

    impl ResourceSource {
        fn parse(self) -> ResourceFile {
            toml::from_str(self.contents).unwrap_or_else(|error| {
                panic!("{} should be valid resource TOML: {error}", self.path)
            })
        }
    }

    macro_rules! resource_sources {
        ($($path:literal),+ $(,)?) => {
            &[$(
                ResourceSource {
                    path: $path,
                    contents: include_str!(concat!("../../", $path)),
                }
            ),+]
        };
    }

    const RESOURCE_SOURCES: &[ResourceSource] = resource_sources![
        "resources/blog-posts.toml",
        "resources/books.toml",
        "resources/cheatsheets.toml",
        "resources/interactive.toml",
        "resources/meta.toml",
        "resources/playlists.toml",
    ];

    #[test]
    fn resource_files_are_valid() {
        let mut has_official_resource = false;

        for source in RESOURCE_SOURCES {
            let resource_file = source.parse();
            assert!(
                !resource_file.resources.is_empty(),
                "{} should contain at least one resource",
                source.path
            );

            for resource in resource_file.resources {
                has_official_resource |= resource.official;

                assert!(
                    !resource.title.trim().is_empty(),
                    "{} contains a resource with an empty title",
                    source.path
                );
                assert!(
                    resource.url.starts_with("https://"),
                    "{} has a non-HTTPS URL for {:?}: {}",
                    source.path,
                    resource.title,
                    resource.url
                );
                assert!(
                    resource
                        .description
                        .as_deref()
                        .is_none_or(|value| !value.trim().is_empty()),
                    "{} has an empty description for {:?}",
                    source.path,
                    resource.title
                );
                assert!(
                    resource
                        .author
                        .as_deref()
                        .is_none_or(|value| !value.trim().is_empty()),
                    "{} has an empty author for {:?}",
                    source.path,
                    resource.title
                );
            }
        }

        assert!(
            has_official_resource,
            "at least one resource should be marked as official"
        );
    }

    #[test]
    fn resource_schema_is_current() {
        assert_schema_is_current::<ResourceFile>("resources/resource.schema.json");
    }
}
