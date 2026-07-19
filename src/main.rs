use dioxus::prelude::*;

mod navbar;
mod pages;

use pages::Route;

#[component]
fn App() -> Element {
    const FAVICON: Asset = asset!("/assets/favicon.ico");
    const FAVICON_16: Asset = asset!("/assets/favicon-16x16.png");
    const FAVICON_32: Asset = asset!("/assets/favicon-32x32.png");
    const APPLE_TOUCH_ICON: Asset = asset!("/assets/apple-touch-icon.png");
    const SITE_WEBMANIFEST: Asset = asset!("/assets/site.webmanifest");
    const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "16x16",
            href: FAVICON_16,
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "32x32",
            href: FAVICON_32,
        }
        document::Link {
            rel: "apple-touch-icon",
            sizes: "180x180",
            href: APPLE_TOUCH_ICON,
        }
        document::Link { rel: "manifest", href: SITE_WEBMANIFEST }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
