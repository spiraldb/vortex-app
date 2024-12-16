use std::{collections::VecDeque, sync::Arc};

use dioxus::{logger::tracing, prelude::*};
use vortex::{stats::ArrayStatistics, validity::ArrayValidity, ArrayDType, ArrayData};

use crate::{
    components::{array_info::EncodingInfo, dtype::DTypeInfo, stats::Statistics, Heading},
    SharedPtr,
};

/// Show some basic info about an ArrayView.
#[component]
pub fn ArrayView(
    file_name: String,
    history_stack: Signal<VecDeque<SharedPtr<ArrayData>>>,
) -> Element {
    // Use the history stack to take data from the front/back of the stack
    let array = history_stack()[0].clone();
    let stats = array.statistics().to_set();

    rsx! {
        div { class: "flex flex-col mt-4",
            // If there are previous history elements, we present a Back button which allows us to
            // pop the stack and go back up to the parent element.
            if history_stack().len() > 1 {
                div {
                    class: "flex flex-row items-center",
                    button {
                        class: "btn btn-primary bg-lime-800 text-lime-400 hover:bg-lime-600/75 px-4 py-2 rounded-md",
                        onclick: move |_| {
                            // Pop the stack to go back to the parent.
                            history_stack.write().pop_front();
                        },
                        "Back"
                    }
                }
            }

            // schema, row_count
            ArraySummary { array: array.clone(), file_name: file_name.clone() }

            div { class: "my-12 h-0.5 border-t-0 bg-neutral-100/30" }

            // Stats.
            Statistics { stats }

            div { class: "my-12 h-0.5 border-t-0 bg-neutral-100/30" }

            DTypeInfo { dtype: array.dtype().clone() }

            div { class: "my-12 h-0.5 border-t-0 bg-neutral-100/30" }

            EncodingInfo { array: array.clone() }

            div { class: "my-12 h-0.5 border-t-0 bg-neutral-100/30" }

            if !array.children().is_empty() {
                ArrayChildren { history_stack }
            }
        }
    }
}

#[component]
fn ArraySummary(array: SharedPtr<ArrayData>, file_name: String) -> Element {
    let size = humansize::format_size(array.nbytes(), humansize::BINARY);
    let row_count = array.len();
    let encoding_id = array.encoding().id().to_string();
    let null_count = array.logical_validity().null_count()?;
    let null_pct: f64 = 100. * (null_count as f64) / (row_count as f64);

    rsx! {
        div {
            Heading { text: "Summary" }

            div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
                table { class: "table-auto w-full min-w-max text-left border-collapse",
                    tbody { class: "border-b border-1 border-zinc-50/10",
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "File Name"
                                }
                            }
                            td { class: "p-1",
                                p { class: "block font-mono text-sm antialiased leading-normal",
                                    "{file_name}"
                                }
                            }
                        }
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Size"
                                }
                            }
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{size}"
                                }
                            }
                        }
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Row Count"
                                }
                            }
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{row_count}"
                                }
                            }
                        }
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Null Count"
                                }
                            }
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{null_count} ({null_pct}%)"
                                }
                            }
                        }
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Encoding"
                                }
                            }
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{encoding_id}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ArrayChildren(mut history_stack: Signal<VecDeque<SharedPtr<ArrayData>>>) -> Element {
    let array = history_stack()[0].clone();
    rsx! {
        Heading { text: "Child Arrays" }

        p {
            class: "p-4 font-regular font-sans text-sm italic text-slate-300/30",
            "Click one of the Child Arrays to explore further"
        }

        table { class: "table-auto w-full min-w-max max-h-96 overflow-y-scroll text-left border-collapse",
            tbody { class: "border-b border-1 border-zinc-50/10",
                for (idx , child) in array.children().iter().cloned().enumerate() {
                    tr {
                        class: "font-normal border-b border-1 border-zinc-50/10",
                        // Interactivity
                        class: "cursor-pointer",
                        // Hover state
                        class: "hover:bg-neutral-800/75 hover:font-bold hover:text-sky-500",
                        onclick: move |_| {
                            let child = child.clone();
                            tracing::info!("descending into the {idx} child");
                            history_stack
                                .write()
                                .push_front(SharedPtr(Arc::new(child)));
                        },
                        td { class: "p-2",
                            p { class: "block font-sans text-sm antialiased leading-normal",
                                "Chunk {idx}"
                            }
                        }
                        td { class: "p-2",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{child.len()} rows"
                            }
                        }
                        td { class: "p-2",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{humansize::format_size(child.nbytes(), humansize::BINARY)}"
                            }
                        }
                    }
                }
            }
        }
    }
}
