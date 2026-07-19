use dioxus::prelude::*;

mod about;
mod home;
mod navbar;
mod resources;
mod schedule;
mod semesters;

use about::About;
use home::Home;
use navbar::Navbar;
use resources::Resources;
use schedule::Schedule;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const FAVICON_16: Asset = asset!("/assets/favicon-16x16.png");
const FAVICON_32: Asset = asset!("/assets/favicon-32x32.png");
const APPLE_TOUCH_ICON: Asset = asset!("/assets/apple-touch-icon.png");
const SITE_WEBMANIFEST: Asset = asset!("/assets/site.webmanifest");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/resources")]
        Resources {},
        #[route("/schedule")]
        Schedule {},
}

#[component]
fn App() -> Element {
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
