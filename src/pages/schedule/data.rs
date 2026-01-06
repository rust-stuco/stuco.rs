use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Week {
    pub title: String,
    pub slides: String,
    rustlings: Option<Vec<String>>,
    book_chapters: Option<Vec<String>>,
    videos: Option<Vec<VideoGroup>>,
    pub homework: Option<Homework>,
    pub homework_ec: Option<Homework>,
    extras: Option<Vec<Extra>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct VideoGroup {
    title: String,
    items: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
pub struct Homework {
    pub name: String,
    pub slug: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct Extra {
    title: String,
    url: String,
}

pub fn load_weeks() -> Vec<Week> {
    let files = [
        include_str!("../../../schedule/week01.toml"),
        include_str!("../../../schedule/week02.toml"),
        include_str!("../../../schedule/week03.toml"),
        include_str!("../../../schedule/week04.toml"),
        include_str!("../../../schedule/week05.toml"),
        include_str!("../../../schedule/week06.toml"),
        include_str!("../../../schedule/week07.toml"),
        include_str!("../../../schedule/week08.toml"),
        include_str!("../../../schedule/week09.toml"),
        include_str!("../../../schedule/week10.toml"),
        include_str!("../../../schedule/week11.toml"),
        include_str!("../../../schedule/week12.toml"),
        include_str!("../../../schedule/week13.toml"),
    ];

    files
        .iter()
        .map(|s| toml::from_str::<Week>(s).expect("Invalid TOML"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_schema() {
        let schema = schemars::schema_for!(Week);
        let json = serde_json::to_string_pretty(&schema).unwrap();
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/schedule/week.schema.json");
        std::fs::write(path, json).unwrap();
    }
}
