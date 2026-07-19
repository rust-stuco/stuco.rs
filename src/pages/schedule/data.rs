use std::sync::LazyLock;

use super::week::Week;

// Update this as the semester progresses.
const LAST_WEEK_SHOWN: usize = 14;

pub(super) static WEEKS: LazyLock<Vec<Week>> = LazyLock::new(load_weeks);

#[derive(Debug, Clone, Copy)]
struct WeekSource {
    path: &'static str,
    contents: &'static str,
}

impl WeekSource {
    fn parse(self) -> Week {
        toml::from_str(self.contents)
            .unwrap_or_else(|error| panic!("{} should be valid week TOML: {error}", self.path))
    }
}

macro_rules! week_sources {
    ($($path:literal),+ $(,)?) => {
        &[$(
            WeekSource {
                path: $path,
                contents: include_str!(concat!("../../../", $path)),
            }
        ),+]
    };
}

const WEEK_SOURCES: &[WeekSource] = week_sources![
    "schedule/week01.toml",
    "schedule/week02.toml",
    "schedule/week03.toml",
    "schedule/week04.toml",
    "schedule/week05.toml",
    "schedule/week06.toml",
    "schedule/week07.toml",
    "schedule/week08.toml",
    "schedule/week09.toml",
    "schedule/week10.toml",
    "schedule/week11.toml",
    "schedule/week12.toml",
    "schedule/week13.toml",
    "schedule/week14.toml",
];

fn load_weeks() -> Vec<Week> {
    WEEK_SOURCES.iter().map(|source| source.parse()).collect()
}

pub(super) fn week_is_published(week_number: usize) -> bool {
    cfg!(debug_assertions) || week_number <= LAST_WEEK_SHOWN
}

fn rustling_order(name: &str) -> Option<u8> {
    Some(match name {
        "intro" => 0,
        "variables" => 1,
        "functions" => 2,
        "if" => 3,
        "primitive_types" => 4,
        "vecs" => 5,
        "move_semantics" => 6,
        "structs" => 7,
        "enums" => 8,
        "strings" => 9,
        "modules" => 10,
        "hashmaps" => 11,
        "options" => 12,
        "error_handling" => 13,
        "generics" => 14,
        "traits" => 15,
        "lifetimes" => 16,
        "tests" => 17,
        "iterators" => 18,
        "smart_pointers" => 19,
        "threads" => 20,
        "macros" => 21,
        "clippy" => 22,
        "conversions" => 23,
        _ => return None,
    })
}

pub(super) fn rustling_url(name: &str) -> String {
    rustling_order(name).map_or_else(
        || "https://github.com/rust-lang/rustlings/tree/main/exercises".to_owned(),
        |order| {
            format!("https://github.com/rust-lang/rustlings/tree/main/exercises/{order:02}_{name}")
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::assert_schema_is_current;

    #[test]
    fn rustlings_groups_are_known() {
        for source in WEEK_SOURCES {
            let week = source.parse();

            for group in week.materials.rustlings.iter().flatten() {
                assert!(
                    rustling_order(group).is_some(),
                    "{} references an unknown Rustlings group: {group:?}",
                    source.path
                );
            }
        }
    }

    #[test]
    fn week_schema_is_current() {
        assert_schema_is_current::<Week>("schedule/week.schema.json");
    }
}
