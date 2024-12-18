use dioxus::prelude::*;

use crate::LOGO;

pub mod array;
pub mod array_info;
pub mod dtype;
pub mod stats;

#[component]
pub fn AppHeader() -> Element {
    rsx! {
        div { class: "flex flex-row items-center",
            img { class: "max-w-8 max-h-8", src: LOGO }
            p { class: "mx-4 font-sans text-2xl", "Vortex File Explorer" }
        }
    }
}

#[component]
pub fn Heading(text: String) -> Element {
    rsx! {
        p { class: "text-2xl font-sans text-white py-4", "{text}" }
    }
}

#[component]
pub fn ErrorMessage(error: String) -> Element {
    rsx! {
        Heading { text: "Error" }
        p { class: "w-full rounded font-mono text-red-700 whitespace-pre-wrap p-10 rounded-md border border-red-700",
            {error}
        }
    }
}
