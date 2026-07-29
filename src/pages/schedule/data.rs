use std::sync::LazyLock;

use jiff::civil::{Date, Time};
use toml::value::Datetime;

use super::week::{Semester, Week};

/// Every `schedule/weeks/*.toml`, in `WEEK_SOURCES` order. `SEMESTER` selects this term's subset.
static LIBRARY: LazyLock<Vec<Week>> = LazyLock::new(load_library);

/// The weeks shown this semester, in schedule order (a subset of `LIBRARY`).
pub(super) static WEEKS: LazyLock<Vec<Scheduled>> = LazyLock::new(load_schedule);

/// A week on this semester's schedule, paired with the date and instant it unlocks.
pub(super) struct Scheduled {
    pub(super) week: &'static Week,
    pub(super) date: Date,
    /// Absolute reveal instant, in epoch milliseconds.
    reveal_ms: i64,
}

impl Scheduled {
    /// Whether this week has unlocked at `now_ms` (epoch milliseconds).
    pub(super) fn is_revealed(&self, now_ms: i64) -> bool {
        now_ms >= self.reveal_ms
    }
}

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

fn load_schedule() -> Vec<Scheduled> {
    let reveal_time = parse_reveal_time(&SEMESTER.reveal_time);

    SEMESTER
        .weeks
        .iter()
        .map(|scheduled| {
            let slug = scheduled.week.as_str();
            let index = WEEK_SOURCES
                .iter()
                .position(|source| source.slug == slug)
                .unwrap_or_else(|| {
                    panic!("schedule/semester.toml references unknown week {slug:?}")
                });

            let date = civil_date(slug, scheduled.date);

            Scheduled {
                week: &LIBRARY[index],
                date,
                reveal_ms: reveal_ms(&SEMESTER.timezone, date, reveal_time),
            }
        })
        .collect()
}

/// Converts a TOML date, naming `entry` if it is anything but a plain calendar date. A date
/// carrying a time component would silently ignore `reveal_time`.
fn civil_date(entry: &str, value: Datetime) -> Date {
    let parsed = value
        .date
        .filter(|_| value.time.is_none())
        .unwrap_or_else(|| {
            panic!("schedule/semester.toml: {entry:?} must be a plain calendar date")
        });

    Date::new(parsed.year as i16, parsed.month as i8, parsed.day as i8).unwrap_or_else(|error| {
        panic!("schedule/semester.toml: {entry:?} is not a valid date: {error}")
    })
}

/// Parses a `"HH:MM"` 24-hour reveal time, panicking on malformed input.
fn parse_reveal_time(text: &str) -> Time {
    Time::strptime("%H:%M", text)
        .unwrap_or_else(|error| panic!("reveal_time {text:?} must be formatted as HH:MM: {error}"))
}

/// Clamps a wait to what `setTimeout` accepts. `gloo-timers` casts its `u32` to `i32`, so a longer
/// wait would turn negative and fire immediately; callers re-arm until the instant arrives.
pub(super) fn timeout_ms(wait_ms: i64) -> u32 {
    wait_ms.clamp(0, i32::MAX.into()) as u32
}

/// The next reveal instant strictly after `now_ms`, if any weeks are still hidden.
pub(super) fn next_reveal_ms(now_ms: i64) -> Option<i64> {
    WEEKS
        .iter()
        .map(|scheduled| scheduled.reveal_ms)
        .find(|&reveal_ms| reveal_ms > now_ms)
}

/// Display name of the current semester, e.g. `"Fall 2026"`.
pub(super) fn semester_name() -> &'static str {
    &SEMESTER.name
}

/// Resolves a wall-clock reveal in an IANA time zone to epoch milliseconds, honoring daylight
/// saving time via `jiff`'s tz database.
fn reveal_ms(timezone: &str, date: Date, time: Time) -> i64 {
    date.to_datetime(time)
        .in_tz(timezone)
        .unwrap_or_else(|error| {
            panic!(
                "schedule/semester.toml: cannot resolve a reveal in time zone {timezone:?}: {error}"
            )
        })
        .timestamp()
        .as_millisecond()
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
    use std::path::Path;

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
        let instant = reveal_ms("America/New_York", Date::constant(2026, 9, 2), at_8pm);
        assert_eq!(instant, 1_788_393_600_000);

        // 2026-10-31 20:00 EDT is the last reveal before the Nov 1 DST change.
        let instant = reveal_ms("America/New_York", Date::constant(2026, 10, 31), at_8pm);
        assert_eq!(instant, 1_793_491_200_000);
    }

    #[test]
    fn reveal_resolves_est_instants() {
        let at_8pm = parse_reveal_time("20:00");

        // The first Sunday of November (2026-11-01) is already EST (UTC-5) at 20:00.
        let instant = reveal_ms("America/New_York", Date::constant(2026, 11, 1), at_8pm);
        assert_eq!(instant, 1_793_581_200_000);

        // 2026-11-04 20:00 EST == 2026-11-05 01:00:00 UTC.
        let instant = reveal_ms("America/New_York", Date::constant(2026, 11, 4), at_8pm);
        assert_eq!(instant, 1_793_840_400_000);
    }

    #[test]
    fn reveal_is_reached_at_or_after_its_instant() {
        let first = &WEEKS[0];

        assert!(!first.is_revealed(first.reveal_ms - 1));
        assert!(first.is_revealed(first.reveal_ms));
        assert!(first.is_revealed(first.reveal_ms + 1));
    }

    #[test]
    fn timeout_clamps_to_the_browser_limit() {
        assert_eq!(timeout_ms(-1), 0);
        assert_eq!(timeout_ms(5_000), 5_000);
        // A 30-day wait, as the first reveal of a semester is: must not overflow `setTimeout`.
        assert_eq!(timeout_ms(30 * 24 * 60 * 60 * 1_000), i32::MAX as u32);
    }

    #[test]
    fn next_reveal_follows_the_current_moment() {
        let (first, second) = (WEEKS[0].reveal_ms, WEEKS[1].reveal_ms);
        let last = WEEKS
            .last()
            .expect("a semester should schedule weeks")
            .reveal_ms;

        assert_eq!(next_reveal_ms(first - 1), Some(first));
        assert_eq!(next_reveal_ms(first), Some(second));
        assert_eq!(next_reveal_ms(last), None);
    }

    #[test]
    fn week_sources_cover_the_content_directory() {
        let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("schedule/weeks");
        let entries = std::fs::read_dir(directory).expect("schedule/weeks should be readable");

        let mut found: Vec<String> = entries
            .filter_map(|entry| {
                let name = entry
                    .expect("schedule/weeks should be readable")
                    .file_name();
                name.to_string_lossy()
                    .strip_suffix(".toml")
                    .map(str::to_owned)
            })
            .collect();
        found.sort_unstable();

        let mut registered: Vec<&str> = WEEK_SOURCES.iter().map(|source| source.slug).collect();
        registered.sort_unstable();

        assert_eq!(
            found, registered,
            "every schedule/weeks/*.toml should be listed in WEEK_SOURCES"
        );
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
            WEEKS
                .windows(2)
                .all(|pair| pair[0].reveal_ms < pair[1].reveal_ms),
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
