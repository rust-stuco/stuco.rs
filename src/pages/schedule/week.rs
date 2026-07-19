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
