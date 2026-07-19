pub(super) const DEFAULT_VIEWPORT_HEIGHT: f64 = 800.0;
pub(super) const OPEN_UPWARD_THRESHOLD: f64 = 0.6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VideoColors {
    pub(super) background: &'static str,
    pub(super) border: &'static str,
    pub(super) text: &'static str,
}

impl VideoColors {
    const fn new(background: &'static str, border: &'static str, text: &'static str) -> Self {
        Self {
            background,
            border,
            text,
        }
    }
}

pub(super) fn video_colors(title: &str) -> VideoColors {
    match title {
        "No Boilerplate Rust Talks" => VideoColors::new("#F59E0B20", "#F59E0B", "#D97706"),
        "The Rust Programming Language Book" => VideoColors::new("#F8717120", "#F87171", "#F87171"),
        "Connor's Lectures" => VideoColors::new("#8B5CF620", "#8B5CF6", "#A78BFA"),
        "Code to the Moon" => VideoColors::new("#3B82F620", "#3B82F6", "#60A5FA"),
        "Crust of Rust" => VideoColors::new("#06B6D420", "#06B6D4", "#22D3EE"),
        "Idiomatic Rust" => VideoColors::new("#10B98120", "#10B981", "#34D399"),
        "Low Level Learning Rust Talks" => VideoColors::new("#EC489920", "#EC4899", "#F472B6"),
        _ => VideoColors::new("#6B728020", "#6B7280", "#6B7280"),
    }
}

pub(super) fn book_chapter_label(slug: &str) -> String {
    let parsed = slug
        .strip_prefix("ch")
        .and_then(|slug| slug.split_once('-'))
        .and_then(|(chapter, remainder)| {
            let section = remainder.split('-').next()?;
            Some((chapter.parse::<u16>().ok()?, section.parse::<u16>().ok()?))
        });

    parsed
        .map(|(chapter, section)| format!("{chapter}.{section}"))
        .unwrap_or_else(|| slug.to_owned())
}

pub(super) fn slide_name(slides: &str) -> &str {
    slides.split_once('_').map_or(slides, |(_, name)| name)
}
