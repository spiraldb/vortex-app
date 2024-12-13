use dioxus::prelude::*;

pub mod array;

#[component]
pub fn Heading(text: String) -> Element {
    rsx! {
        p { class: "text-2xl font-sans text-white py-4",
            "{text}"
        }
    }
}
