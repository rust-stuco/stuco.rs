use dioxus::prelude::*;

#[component]
pub fn Resources() -> Element {
    rsx! {
        document::Title { "Resources - Rust StuCo" }
        div { class: "max-w-prose mx-auto px-8 pt-16",
            h1 { class: "text-3xl font-bold italic text-primary mb-6 text-center",
                "Resources"
            }
        }
    }
}
