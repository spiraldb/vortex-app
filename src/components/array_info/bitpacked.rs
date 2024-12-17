use dioxus::prelude::*;
use vortex::fastlanes::BitPackedArray;

use crate::{components::Heading, SharedPtr};

/// Encoding info for `BitPackedEncoding`.
#[component]
pub fn BitPackedInfo(array: SharedPtr<BitPackedArray>) -> Element {
    let bit_width = array.bit_width();
    rsx! {
        Heading { text: "FastLanes Bit-packed Encoding" }

        div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
            table { class: "table-auto w-full min-w-max text-left border-collapse",
                tbody { class: "border-b border-1 border-zinc-50/10",
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Bit-Width"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{bit_width}"
                            }
                        }
                    }
                }
            }
        }
    }
}
