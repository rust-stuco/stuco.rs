use std::sync::LazyLock;

use super::week::{Semester, Week};

/// Every `schedule/weeks/*.toml`, in `WEEK_SOURCES` order. `SEMESTER` selects this term's subset.
static LIBRARY: LazyLock<Vec<Week>> = LazyLock::new(load_library);

/// The weeks shown this semester, in schedule order (a subset of `LIBRARY`).
pub(super) static WEEKS: LazyLock<Vec<&'static Week>> = LazyLock::new(|| {
    SEMESTER
        .weeks
        .iter()
        .map(|scheduled| {
            let index = WEEK_SOURCES
                .iter()
                .position(|source| source.slug == scheduled.week)
                .unwrap_or_else(|| {
                    panic!(
                        "schedule/semester.toml references unknown week {:?}",
                        scheduled.week
                    )
                });
            &LIBRARY[index]
        })
        .collect()
});

#[derive(Debug, Clone, Copy)]
struct WeekSource {
    slug: &'static str,
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
    ($($slug:literal),+ $(,)?) => {
        &[$(
            WeekSource {
                slug: $slug,
                path: concat!("schedule/weeks/", $slug, ".toml"),
                contents: include_str!(concat!("../../../schedule/weeks/", $slug, ".toml")),
            }
        ),+]
    };
}

const WEEK_SOURCES: &[WeekSource] = week_sources![
    "introduction",
    "ownership_p1",
    "structs_enums",
    "collections_generics",
    "errors_traits",
    "modules_testing",
    "ecosystem",
    "closures_iterators",
    "ownership_p2",
    "lifetimes",
    "smart_pointers",
    "unsafe",
    "parallelism",
    "concurrency",
];

fn load_library() -> Vec<Week> {
    WEEK_SOURCES.iter().map(|source| source.parse()).collect()
}

const SEMESTER_SOURCE: &str = include_str!("../../../schedule/semester.toml");

static SEMESTER: LazyLock<Semester> = LazyLock::new(|| {
    toml::from_str(SEMESTER_SOURCE)
        .unwrap_or_else(|error| panic!("schedule/semester.toml should be valid TOML: {error}"))
});

/// Absolute reveal instant (epoch milliseconds) for each curriculum week, in order.
static REVEAL_MS: LazyLock<Vec<i64>> = LazyLock::new(compute_reveal_ms);

fn compute_reveal_ms() -> Vec<i64> {
    let reveal_time = parse_reveal_time(&SEMESTER.reveal_time);

    SEMESTER
        .weeks
        .iter()
        .map(|scheduled| {
            let date = scheduled
                .date
                .date
                .unwrap_or_else(|| panic!("schedule/semester.toml entries must be calendar dates"));
            reveal_ms(
                &SEMESTER.timezone,
                date.year as i16,
                date.month as i8,
                date.day as i8,
                reveal_time,
            )
        })
        .collect()
}

/// Parses a `"HH:MM"` 24-hour reveal time, panicking on malformed input.
fn parse_reveal_time(text: &str) -> jiff::civil::Time {
    jiff::civil::Time::strptime("%H:%M", text)
        .unwrap_or_else(|error| panic!("reveal_time {text:?} must be formatted as HH:MM: {error}"))
}

/// Whether week `week_index` (zero-based) has been revealed at `now_ms` (epoch milliseconds).
pub(super) fn is_revealed(week_index: usize, now_ms: i64) -> bool {
    REVEAL_MS
        .get(week_index)
        .is_some_and(|&reveal_ms| revealed_at(reveal_ms, now_ms))
}

/// Display name of the current semester, e.g. `"Fall 2026"`.
pub(super) fn semester_name() -> &'static str {
    &SEMESTER.name
}

/// Resolves a wall-clock reveal in an IANA time zone to epoch milliseconds, honoring daylight
/// saving time via `jiff`'s tz database.
fn reveal_ms(timezone: &str, year: i16, month: i8, day: i8, time: jiff::civil::Time) -> i64 {
    jiff::civil::date(year, month, day)
        .to_datetime(time)
        .in_tz(timezone)
        .unwrap_or_else(|error| {
            panic!(
                "schedule/semester.toml: cannot resolve a reveal in time zone {timezone:?}: {error}"
            )
        })
        .timestamp()
        .as_millisecond()
}

/// Whether a reveal instant has been reached at the given moment (both epoch milliseconds).
fn revealed_at(reveal_ms: i64, now_ms: i64) -> bool {
    now_ms >= reveal_ms
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

    #[test]
    fn semester_schema_is_current() {
        assert_schema_is_current::<Semester>("schedule/semester.schema.json");
    }

    #[test]
    fn reveal_resolves_edt_instants() {
        let at_8pm = parse_reveal_time("20:00");

        // 2026-09-02 20:00 EDT (UTC-4) == 2026-09-03 00:00:00 UTC.
        assert_eq!(
            reveal_ms("America/New_York", 2026, 9, 2, at_8pm),
            1_788_393_600_000
        );
        // 2026-10-31 20:00 EDT is the last reveal before the Nov 1 DST change.
        assert_eq!(
            reveal_ms("America/New_York", 2026, 10, 31, at_8pm),
            1_793_491_200_000
        );
    }

    #[test]
    fn reveal_resolves_est_instants() {
        let at_8pm = parse_reveal_time("20:00");

        // The first Sunday of November (2026-11-01) is already EST (UTC-5) at 20:00.
        assert_eq!(
            reveal_ms("America/New_York", 2026, 11, 1, at_8pm),
            1_793_581_200_000
        );
        // 2026-11-04 20:00 EST == 2026-11-05 01:00:00 UTC.
        assert_eq!(
            reveal_ms("America/New_York", 2026, 11, 4, at_8pm),
            1_793_840_400_000
        );
    }

    #[test]
    fn reveal_is_reached_at_or_after_its_instant() {
        assert!(!revealed_at(1_000, 999));
        assert!(revealed_at(1_000, 1_000));
        assert!(revealed_at(1_000, 1_001));
    }

    #[test]
    fn semester_schedule_is_valid() {
        // Forcing WEEKS panics if any scheduled week has no matching content file.
        assert_eq!(
            WEEKS.len(),
            SEMESTER.weeks.len(),
            "every scheduled week should resolve to a content file"
        );
        assert!(
            REVEAL_MS.windows(2).all(|pair| pair[0] < pair[1]),
            "schedule/semester.toml reveal dates should be strictly increasing"
        );

        let mut slugs: Vec<&str> = SEMESTER
            .weeks
            .iter()
            .map(|week| week.week.as_str())
            .collect();
        let scheduled = slugs.len();
        slugs.sort_unstable();
        slugs.dedup();
        assert_eq!(
            slugs.len(),
            scheduled,
            "a week should not be scheduled twice"
        );
    }
}
