use schemars::JsonSchema;
use serde::Deserialize;

// For TOML deserialization + schema
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

// For runtime use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Books,
    BlogPosts,
    Cheatsheets,
    Interactive,
    Meta,
    Playlists,
}

#[derive(Debug)]
pub struct Resource {
    title: String,
    url: String,
    description: Option<String>,
    author: Option<String>,
    official: Option<bool>,
    category: Category,
}

pub fn load_resources() -> Vec<Resource> {
    let files: &[(&str, Category)] = &[
        (
            include_str!("../../../resources/blog-posts.toml"),
            Category::BlogPosts,
        ),
        (
            include_str!("../../../resources/books.toml"),
            Category::Books,
        ),
        (
            include_str!("../../../resources/cheatsheets.toml"),
            Category::Cheatsheets,
        ),
        (
            include_str!("../../../resources/interactive.toml"),
            Category::Interactive,
        ),
        (include_str!("../../../resources/meta.toml"), Category::Meta),
        (
            include_str!("../../../resources/playlists.toml"),
            Category::Playlists,
        ),
    ];

    files
        .iter()
        .flat_map(|(s, cat)| {
            toml::from_str::<ResourceFile>(s)
                .expect("Invalid TOML")
                .resources
                .into_iter()
                .map(|r| Resource {
                    title: r.title,
                    url: r.url,
                    description: r.description,
                    author: r.author,
                    official: r.official,
                    category: *cat,
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_schema() {
        let schema = schemars::schema_for!(ResourceFile);
        let json = serde_json::to_string_pretty(&schema).unwrap();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/resource.schema.json"
        );
        std::fs::write(path, json).unwrap();
    }
}
