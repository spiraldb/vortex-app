use dioxus::prelude::*;
use vortex::{compute::scalar_at, dict::DictArray, ArrayDType};

use crate::{components::Heading, SharedPtr};

#[component]
pub fn DictInfo(array: SharedPtr<DictArray>) -> Element {
    let mut show_dictionary = use_signal(|| false);
    let num_values = array.values().len();
    rsx! {
        Heading { text: "Dictionary Encoding" }

        a {
            // Styling.
            class: "text-lg flex flex-row cursor-pointer gap-x-1",

            // Event handlers.
            onclick: move |_| {
                show_dictionary.toggle();
            },

            if show_dictionary() {
                span { "▼  " }
            } else {
                span { "►  " }
            }

            "Dictionary ({num_values} values)"
        }

        if show_dictionary() {
            DictionaryView { array }
        }
    }
}
#[component]
pub fn DictionaryView(array: SharedPtr<DictArray>) -> Element {
    let dict_values = array.values();
    // Turn each of the dict values into a block of strings.
    let mut dict_strings = Vec::with_capacity(dict_values.len());
    // DictArray code 0 is reserved for NULL for nullable arrays, it is a valid code.
    if array.dtype().is_nullable() {
        dict_strings.push("null".to_string());
    }

    for i in 0..dict_values.len() {
        let value = scalar_at(&dict_values, i)?;
        dict_strings.push(format!("{value}"));
    }

    rsx! {
        div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
            table { class: "table-auto w-full min-w-max text-left border-collapse",
                thead { class: "bg-neutral-700 border-b border-1 border-zinc-50/10",
                    tr {
                        th { class: "p-4",
                            p { class: "block font-sans text-sm antialiased font-normal leading-none opacity-70",
                                "Code"
                            }
                        }
                        th { class: "p-4",
                            p { class: "block font-sans text-sm antialiased font-normal leading-none opacity-70",
                                "Value"
                            }
                        }
                    }
                }

                tbody { class: "border-b border-1 border-zinc-50/10",
                    for (code , value) in dict_strings.into_iter().enumerate() {
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{code}"
                                }
                            }
                            td { class: "p-1",
                                p { class: "block font-mono text-sm antialiased leading-normal",
                                    "{value}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
