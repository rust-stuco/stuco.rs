use serde::Deserialize;
use toml::value::Datetime;

#[cfg(test)]
use schemars::JsonSchema;

#[derive(Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct Week {
    pub(super) title: String,
    pub(super) slides: String,
    #[serde(flatten)]
    pub(super) materials: Materials,
    #[serde(flatten)]
    pub(super) assignments: Assignments,
}

/// This semester's schedule: which content weeks run, in what order, and when each unlocks.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct Semester {
    pub(super) name: String,
    /// Local time each week's homework unlocks, as `"HH:MM"` (24-hour).
    pub(super) reveal_time: String,
    /// Minutes before `reveal_time` when each week's slides unlock.
    pub(super) slides_reveal_minutes_before: u16,
    /// IANA time zone the reveal time is expressed in, e.g. `"America/New_York"`.
    pub(super) timezone: String,
    /// The weeks shown this term, in schedule order.
    pub(super) weeks: Vec<ScheduledWeek>,
    /// Class dates with no content, such as university breaks.
    #[serde(default)]
    pub(super) breaks: Vec<Break>,
}

/// One entry in a semester schedule: which content week to show, and when it unlocks.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct ScheduledWeek {
    /// The week's slug: the file stem under `schedule/weeks/`.
    pub(super) week: String,
    /// Calendar date the week's homework unlocks.
    #[cfg_attr(test, schemars(with = "String"))]
    pub(super) date: Datetime,
}

/// A skipped class, shown in schedule order with no slides or homework.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct Break {
    /// Name shown in place of a week's topics, e.g. `"Fall Break"`.
    pub(super) name: String,
    /// Calendar date of the skipped class.
    #[cfg_attr(test, schemars(with = "String"))]
    pub(super) date: Datetime,
}

#[derive(Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct Materials {
    pub(super) rustlings: Option<Vec<String>>,
    pub(super) book_chapters: Option<Vec<String>>,
    pub(super) videos: Option<Vec<VideoGroup>>,
    pub(super) extras: Option<Vec<Extra>>,
}

impl Materials {
    pub(super) fn has_any(&self) -> bool {
        self.rustlings
            .as_ref()
            .is_some_and(|items| !items.is_empty())
            || self
                .book_chapters
                .as_ref()
                .is_some_and(|items| !items.is_empty())
            || self.videos.as_ref().is_some_and(|items| !items.is_empty())
            || self.extras.as_ref().is_some_and(|items| !items.is_empty())
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct Assignments {
    #[serde(rename = "homework")]
    pub(super) primary: Option<Homework>,
    #[serde(rename = "homework_ec")]
    pub(super) extra_credit: Option<Homework>,
    #[serde(rename = "homework_alt")]
    pub(super) alternative: Option<Homework>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct VideoGroup {
    pub(super) title: String,
    pub(super) items: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct Homework {
    pub(super) name: String,
    pub(super) slug: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct Extra {
    pub(super) title: String,
    pub(super) url: String,
}
