use dioxus::prelude::*;

use crate::LOGO;

pub mod array;

#[component]
pub fn AppHeader() -> Element {
    rsx! {
        div { class: "flex flex-row items-center",
            img { class: "max-w-8 max-h-8", src: LOGO }
            p { class: "mx-4 font-serif text-2xl", "Vortex File Explorer" }
        }
    }
}

#[component]
pub fn Heading(text: String) -> Element {
    rsx! {
        p { class: "text-2xl font-sans text-white py-4", "{text}" }
    }
}