use serde::Deserialize;

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

/// The current semester's schedule: which content weeks run this term, in order, and when each
/// unlocks. Weeks are selected from the content library by slug, so reordering or dropping a week
/// is a config edit, never a code change.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct Semester {
    pub(super) name: String,
    /// Local time each week unlocks, as `"HH:MM"` (24-hour).
    pub(super) reveal_time: String,
    /// IANA time zone the reveal time is expressed in, e.g. `"America/New_York"`.
    pub(super) timezone: String,
    /// The weeks shown this term, in schedule order.
    pub(super) weeks: Vec<ScheduledWeek>,
}

/// One entry in a semester schedule: which content week to show, and when it unlocks.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(JsonSchema))]
pub(super) struct ScheduledWeek {
    /// Slug of the content week — the file stem under `schedule/weeks/` (e.g. `"ecosystem"`).
    pub(super) week: String,
    /// Calendar date the week's slides and homework unlock.
    #[cfg_attr(test, schemars(with = "String"))]
    pub(super) date: toml::value::Datetime,
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
