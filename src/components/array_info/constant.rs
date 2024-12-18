use dioxus::prelude::*;
use vortex::array::ConstantArray;

use crate::SharedPtr;

#[component]
pub fn ConstantInfo(array: SharedPtr<ConstantArray>) -> Element {
    let const_value = array.scalar();

    rsx! {
        div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
            table { class: "table-auto w-full min-w-max text-left border-collapse",
                tbody { class: "border-b border-1 border-zinc-50/10",
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Constant Value"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{const_value}"
                            }
                        }
                    }
                }
            }
        }
    }
}
