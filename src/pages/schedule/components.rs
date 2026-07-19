use dioxus::prelude::*;

use super::data::{WEEKS, rustling_url, week_is_published};
use super::display::{
    DEFAULT_VIEWPORT_HEIGHT, OPEN_UPWARD_THRESHOLD, VideoColors, book_chapter_label, slide_name,
    video_colors,
};
use super::week::{Extra, Homework, Materials, VideoGroup, Week};

#[component]
pub(crate) fn Schedule() -> Element {
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

    let published = week_is_published(week_num);
    let assignments = &week.assignments;

    let handle_click = move |_| {
        if expanded() {
            expanded.set(false);
        } else {
            // Open the menu upward when there is insufficient space below it.
            spawn(async move {
                if let Some(mounted) = button_ref()
                    && let Ok(rect) = mounted.get_client_rect().await
                {
                    let viewport_height = web_sys::window()
                        .and_then(|w| w.inner_height().ok())
                        .and_then(|h| h.as_f64())
                        .unwrap_or(DEFAULT_VIEWPORT_HEIGHT);

                    open_upward.set(rect.origin.y > viewport_height * OPEN_UPWARD_THRESHOLD);
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
                if week.materials.has_any() {
                    div { class: "inline-block relative ml-2",
                        button {
                            r#type: "button",
                            class: "text-secondary hover:text-primary text-sm",
                            aria_expanded: expanded(),
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
                                MaterialsDropdown { materials: &week.materials }
                            }
                        }
                    }
                }
            }
            td { class: "p-2 align-top",
                if published {
                    SlideLinks { slides: &week.slides }
                }
            }
            td { class: "p-2 align-top",
                if published {
                    if let Some(homework) = &assignments.primary {
                        HomeworkLinks { homework }
                    }
                    if let Some(homework) = &assignments.extra_credit {
                        div { class: "mt-1",
                            span { class: "text-secondary text-sm", "EC: " }
                            HomeworkLinks { homework }
                        }
                    }
                    if let Some(homework) = &assignments.alternative {
                        div { class: "mt-1",
                            span { class: "text-secondary text-sm", "OR " }
                            HomeworkLinks { homework }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MaterialsDropdown(materials: &'static Materials) -> Element {
    rsx! {
        if let Some(rustlings) = &materials.rustlings {
            div { class: "space-y-3",
                span { class: "text-secondary font-semibold", "Rustlings" }
                div { class: "flex flex-wrap gap-1",
                    for exercise in rustlings {
                        RustlingLink { exercise }
                    }
                }
            }
        }

        if let Some(chapters) = &materials.book_chapters {
            div { class: "space-y-3",
                span { class: "text-secondary font-semibold", "Book Chapters" }
                div { class: "flex flex-wrap gap-1",
                    for chapter in chapters {
                        BookChapterLinks { chapter }
                    }
                }
            }
        }

        if let Some(videos) = &materials.videos {
            div { class: "space-y-3",
                span { class: "text-secondary font-semibold", "Videos" }
                div { class: "flex flex-wrap gap-1",
                    for group in videos {
                        VideoPills { group }
                    }
                }
            }
        }

        if let Some(extras) = &materials.extras {
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
fn VideoPills(group: &'static VideoGroup) -> Element {
    let VideoColors {
        background,
        border,
        text,
    } = video_colors(&group.title);

    rsx! {
        for (i , url) in group.items.iter().enumerate() {
            a {
                class: "inline-flex items-center px-2 py-0.5 rounded text-xs font-medium hover:brightness-110 transition-all border",
                style: "background: {background}; border-color: {border}; color: {text};",
                href: "{url}",
                target: "_blank",
                rel: "noopener noreferrer",
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
    let display = book_chapter_label(chapter);

    let official_url = format!("https://doc.rust-lang.org/book/{chapter}.html");
    let brown_url = format!("https://rust-book.cs.brown.edu/{chapter}.html");

    rsx! {
        span { class: "inline-flex rounded overflow-hidden border border-tertiary/50",
            a {
                class: "px-2 py-0.5 text-xs bg-tertiary/20 hover:bg-tertiary/40 transition-colors",
                href: "{official_url}",
                target: "_blank",
                rel: "noopener noreferrer",
                title: "Official Rust Book",
                "§{display}"
            }
            a {
                class: "px-2 py-0.5 text-xs bg-amber-900/30 hover:bg-amber-900/50 transition-colors border-l border-tertiary/50",
                href: "{brown_url}",
                target: "_blank",
                rel: "noopener noreferrer",
                title: "Brown University Edition",
                "🐻"
            }
        }
    }
}

#[component]
fn RustlingLink(exercise: &'static str) -> Element {
    let url = rustling_url(exercise);

    rsx! {
        a {
            class: "inline-flex items-center px-2 py-0.5 rounded text-xs bg-orange-900/30 hover:bg-orange-900/50 transition-colors",
            href: "{url}",
            target: "_blank",
            rel: "noopener noreferrer",
            "🦀 {exercise}"
        }
    }
}

#[component]
fn SlideLinks(slides: &'static str) -> Element {
    let name = slide_name(slides);

    rsx! {
        div { class: "flex gap-2 text-sm",
            a {
                class: "text-primary hover:underline",
                href: "/lectures/{slides}/{name}-light.pdf",
                target: "_blank",
                rel: "noopener noreferrer",
                "light"
            }
            span { class: "text-secondary", "/" }
            a {
                class: "text-primary hover:underline",
                href: "/lectures/{slides}/{name}-dark.pdf",
                target: "_blank",
                rel: "noopener noreferrer",
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
                    rel: "noopener noreferrer",
                    "handout"
                }
                " / "
                a {
                    class: "text-primary hover:underline",
                    href: "/hw/{slug}/doc/{slug}/index.html",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "writeup"
                }
                ")"
            }
        }
    } else {
        rsx! {
            span { "{homework.name} " }
            span { class: "text-sm", "(Gradescope)" }
        }
    }
}

#[component]
fn ExtraLink(extra: &'static Extra) -> Element {
    rsx! {
        a {
            class: "inline-flex items-center px-2 py-0.5 rounded text-xs bg-tertiary/20 hover:bg-tertiary/40 transition-colors border border-tertiary/50",
            href: "{extra.url}",
            target: "_blank",
            rel: "noopener noreferrer",
            "{extra.title}"
        }
    }
}
