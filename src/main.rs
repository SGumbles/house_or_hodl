use dioxus::{prelude::*};

static CSS : Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx!{
        document::Stylesheet { href : CSS}
        h1 {
            class: "text-lg",
            "Hello"
        }
    }
}
