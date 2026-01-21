use crate::pages::schedule::data::{Homework, Week, load_weeks};
use dioxus::prelude::*;
use std::sync::LazyLock;

// Update this as the semester goes
const LAST_WEEK_SHOWN: usize = if cfg!(debug_assertions) {
    usize::MAX
} else {
    2
};

static WEEKS: LazyLock<Vec<Week>> = LazyLock::new(load_weeks);

fn video_color(title: &str) -> (&'static str, &'static str, &'static str) {
    // (background, border, text color)
    match title {
        "No Boilerplate Rust Talks" => ("#F59E0B20", "#F59E0B", "#D97706"), // amber
        "The Rust Programming Language Book" => ("#F8717120", "#F87171", "#F87171"), // red
        "Connor's Lectures" => ("#8B5CF620", "#8B5CF6", "#A78BFA"),         // violet
        "Code to the Moon" => ("#3B82F620", "#3B82F6", "#60A5FA"),          // blue
        "Crust of Rust" => ("#06B6D420", "#06B6D4", "#22D3EE"),             // cyan
        "Idiomatic Rust" => ("#10B98120", "#10B981", "#34D399"),            // emerald
        "Low Level Learning Rust Talks" => ("#EC489920", "#EC4899", "#F472B6"), // pink
        _ => ("#6B728020", "#6B7280", "#6B7280"),
    }
}

#[component]
pub fn Schedule() -> Element {
    rsx! {
        document::Title { "Schedule - Rust StuCo" }
        div { class: "max-w-4xl mx-auto px-8 pt-16",
            h1 { class: "text-3xl font-bold italic text-primary mb-6 text-center",
                "Schedule"
            }
            table { class: "w-full border-collapse",
                thead {
                    tr { class: "border-b border-tertiary",
                        th { class: "text-left p-2", "Week" }
                        th { class: "text-left p-2", "Topics" }
                        th { class: "text-left p-2", "Slides" }
                        th { class: "text-left p-2", "Homework" }
                    }
                }
                tbody {
                    for (i , week) in WEEKS.iter().enumerate() {
                        WeekRow { week_num: i + 1, week }
                    }
                }
            }
        }
    }
}

#[component]
fn WeekRow(week_num: usize, week: &'static Week) -> Element {
    let mut expanded = use_signal(|| false);
    let mut open_upward = use_signal(|| false);
    let mut button_ref = use_signal(|| None::<std::rc::Rc<MountedData>>);

    let has_materials = week.videos.is_some()
        || week.book_chapters.is_some()
        || week.rustlings.is_some()
        || week.extras.is_some();

    let handle_click = move |_| {
        if expanded() {
            expanded.set(false);
        } else {
            // Check if we should open upward
            spawn(async move {
                if let Some(mounted) = button_ref()
                    && let Ok(rect) = mounted.get_client_rect().await
                {
                    let viewport_height = web_sys::window()
                        .and_then(|w| w.inner_height().ok())
                        .and_then(|h| h.as_f64())
                        .unwrap_or(800.0);

                    // If button is in bottom 40% of viewport, open upward
                    open_upward.set(rect.origin.y > viewport_height * 0.6);
                }
                expanded.set(true);
            });
        }
    };

    rsx! {
        tr { class: "border-b border-tertiary/50",
            td { class: "p-2 align-top", "{week_num}" }
            td { class: "p-2 align-top",
                span { class: "font-semibold", "{week.title}" }
                if has_materials {
                    div { class: "inline-block relative ml-2",
                        button {
                            class: "text-secondary hover:text-primary text-sm",
                            onmounted: move |e| button_ref.set(Some(e.data())),
                            onclick: handle_click,
                            if expanded() {
                                "▲ materials"
                            } else {
                                "▼ materials"
                            }
                        }
                        if expanded() {
                            div {
                                class: "fixed inset-0 z-10",
                                onclick: move |_| expanded.set(false),
                            }
                            div {
                                class: "absolute left-0 z-20 w-96 p-3 bg-background border border-tertiary/50 rounded-lg shadow-lg space-y-3 text-sm",
                                class: if open_upward() { "bottom-full mb-1" } else { "top-full mt-1" },
                                onclick: move |e| e.stop_propagation(),
                                MaterialsDropdown { week }
                            }
                        }
                    }
                }
            }
            td { class: "p-2 align-top",
                if week_num <= LAST_WEEK_SHOWN {
                    SlideLinks { slides: &week.slides }
                }
            }
            td { class: "p-2 align-top",
                if week_num <= LAST_WEEK_SHOWN {
                    if let Some(hw) = &week.homework {
                        HomeworkLinks { homework: hw }
                    }
                    if let Some(hw_ec) = &week.homework_ec {
                        div { class: "mt-1",
                            span { class: "text-secondary text-sm", "EC: " }
                            HomeworkLinks { homework: hw_ec }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MaterialsDropdown(week: &'static Week) -> Element {
    rsx! {
        // Rustlings
        if let Some(rustlings) = &week.rustlings {
            div { class: "space-y-3",
                span { class: "text-secondary font-semibold", "Rustlings" }
                div { class: "flex flex-wrap gap-1",
                    for exercise in rustlings {
                        RustlingLink { exercise }
                    }
                }
            }
        }

        // Book chapters
        if let Some(chapters) = &week.book_chapters {
            div { class: "space-y-3",
                span { class: "text-secondary font-semibold", "Book Chapters" }
                div { class: "flex flex-wrap gap-1",
                    for chapter in chapters {
                        BookChapterLinks { chapter }
                    }
                }
            }
        }

        // Videos
        if let Some(videos) = &week.videos {
            div { class: "space-y-3",
                span { class: "text-secondary font-semibold", "Videos" }
                div { class: "flex flex-wrap gap-1",
                    for group in videos {
                        VideoPills { group }
                    }
                }
            }
        }

        // Extras
        if let Some(extras) = &week.extras {
            div { class: "space-y-3",
                span { class: "text-secondary font-semibold", "Extras" }
                div { class: "flex flex-wrap gap-1",
                    for extra in extras {
                        ExtraLink { extra }
                    }
                }
            }
        }
    }
}

#[component]
fn VideoPills(group: &'static crate::pages::schedule::data::VideoGroup) -> Element {
    let (bg, border, text) = video_color(&group.title);

    rsx! {
        for (i , url) in group.items.iter().enumerate() {
            a {
                class: "inline-flex items-center px-2 py-0.5 rounded text-xs font-medium hover:brightness-110 transition-all border",
                style: "background: {bg}; border-color: {border}; color: {text};",
                href: "{url}",
                target: "_blank",
                if group.items.len() == 1 {
                    "{group.title}"
                } else {
                    "{group.title} #{i + 1}"
                }
            }
        }
    }
}

#[component]
fn BookChapterLinks(chapter: &'static str) -> Element {
    // chapter format: "ch04-01-what-is-ownership" -> "4.1"
    let display = chapter
        .strip_prefix("ch")
        .map(|s| {
            let parts: Vec<&str> = s.split('-').take(2).collect();
            if parts.len() == 2 {
                format!(
                    "{}.{}",
                    parts[0].trim_start_matches('0'),
                    parts[1].trim_start_matches('0')
                )
            } else {
                chapter.to_string()
            }
        })
        .unwrap_or_else(|| chapter.to_string());

    let official_url = format!("https://doc.rust-lang.org/book/{chapter}.html");
    let brown_url = format!("https://rust-book.cs.brown.edu/{chapter}.html");

    rsx! {
        span { class: "inline-flex rounded overflow-hidden border border-tertiary/50",
            a {
                class: "px-2 py-0.5 text-xs bg-tertiary/20 hover:bg-tertiary/40 transition-colors",
                href: "{official_url}",
                target: "_blank",
                title: "Official Rust Book",
                "§{display}"
            }
            a {
                class: "px-2 py-0.5 text-xs bg-amber-900/30 hover:bg-amber-900/50 transition-colors border-l border-tertiary/50",
                href: "{brown_url}",
                target: "_blank",
                title: "Brown University Edition",
                "🐻"
            }
        }
    }
}

#[component]
fn RustlingLink(exercise: &'static str) -> Element {
    let order = rustling_order(exercise);
    let url =
        format!("https://github.com/rust-lang/rustlings/tree/main/exercises/{order:02}_{exercise}");

    rsx! {
        a {
            class: "inline-flex items-center px-2 py-0.5 rounded text-xs bg-orange-900/30 hover:bg-orange-900/50 transition-colors",
            href: "{url}",
            target: "_blank",
            "🦀 {exercise}"
        }
    }
}

fn rustling_order(name: &str) -> usize {
    match name {
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
        _ => 99,
    }
}

#[component]
fn SlideLinks(slides: &'static str) -> Element {
    let name = slides.split('_').skip(1).collect::<Vec<_>>().join("_");

    rsx! {
        div { class: "flex gap-2 text-sm",
            a {
                class: "text-primary hover:underline",
                href: "/lectures/{slides}/{name}-light.pdf",
                target: "_blank",
                "light"
            }
            span { class: "text-secondary", "/" }
            a {
                class: "text-primary hover:underline",
                href: "/lectures/{slides}/{name}-dark.pdf",
                target: "_blank",
                "dark"
            }
        }
    }
}

#[component]
fn HomeworkLinks(homework: &'static Homework) -> Element {
    if let Some(slug) = &homework.slug {
        rsx! {
            span { "{homework.name} " }
            span { class: "text-sm",
                "("
                a {
                    class: "text-primary hover:underline",
                    href: "/hw/{slug}/{slug}.zip",
                    target: "_blank",
                    "handout"
                }
                " / "
                a {
                    class: "text-primary hover:underline",
                    href: "/hw/{slug}/doc/{slug}/index.html",
                    target: "_blank",
                    "writeup"
                }
                ")"
            }
        }
    } else {
        rsx! {
            span { "{homework.name}" }
        }
    }
}

#[component]
fn ExtraLink(extra: &'static crate::pages::schedule::data::Extra) -> Element {
    rsx! {
        a {
            class: "inline-flex items-center px-2 py-0.5 rounded text-xs bg-tertiary/20 hover:bg-tertiary/40 transition-colors border border-tertiary/50",
            href: "{extra.url}",
            target: "_blank",
            "{extra.title}"
        }
    }
}
