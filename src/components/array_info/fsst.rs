use dioxus::prelude::*;
use vortex::{fsst::FSSTArray, IntoArrayVariant};

use crate::{components::Heading, SharedPtr};

/// Information view for `FSSTArray`. Provides access to the symbol table.
#[component]
pub fn FSSTInfo(array: SharedPtr<FSSTArray>) -> Element {
    let mut show_symbol_table = use_signal(|| false);

    rsx! {
        Heading { text: "FSST Encoding" }

        // Short description of the symbol table child
        a {
            // Styling.
            class: "text-lg flex flex-row cursor-pointer gap-x-1",

            // Event handlers.
            onclick: move |_| {
                show_symbol_table.toggle();
            },

            if show_symbol_table() {
                span { "▼  " }
            } else {
                span { "►  " }
            }

            "Symbol Table"
        }

        if show_symbol_table() {
            SymbolTableView { array }
        }
    }
}

#[component]
pub fn SymbolTableView(array: SharedPtr<FSSTArray>) -> Element {
    let symbol_lens = array
        .symbol_lengths()
        .into_primitive()?
        .into_maybe_null_slice::<u8>();
    let symbols = array
        .symbols()
        .into_primitive()?
        .into_maybe_null_slice::<u64>();
    let symbols: Vec<String> = symbols
        .into_iter()
        .zip(symbol_lens.into_iter())
        .map(|(symbol, len)| {
            let symbols: Vec<u8> = symbol.to_le_bytes().into_iter().take(len as _).collect();
            match String::from_utf8(symbols) {
                Ok(s) => s,
                Err(binary) => {
                    let bytes = binary.into_bytes();
                    format!("binary: {bytes:?}")
                }
            }
        })
        .collect();

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
                    for (code , symbol) in symbols.into_iter().enumerate() {
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{code}"
                                }
                            }
                            td { class: "p-1",
                                p { class: "block font-mono text-sm antialiased leading-normal",
                                    "{symbol}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
