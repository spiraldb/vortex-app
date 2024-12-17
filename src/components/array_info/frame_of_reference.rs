use dioxus::prelude::*;
use vortex::fastlanes::FoRArray;

use crate::{components::Heading, SharedPtr};

/// Extra information for the Frame of Reference (FoR) encoding.
#[component]
pub fn FrameOfReferenceInfo(array: SharedPtr<FoRArray>) -> Element {
    let reference = array.reference_scalar();
    let shift = array.shift();

    rsx! {
        Heading { text: "FastLanes Frame-of-reference Encoding" }

        div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
            table { class: "table-auto w-full min-w-max text-left border-collapse",
                tbody { class: "border-b border-1 border-zinc-50/10",
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Reference"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{reference}"
                            }
                        }
                    }
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Shift"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{shift}"
                            }
                        }
                    }
                }
            }
        }
    }
}
