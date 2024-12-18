use crate::components::Heading;
use crate::SharedPtr;
use dioxus::prelude::*;
use vortex::alp::{ALPArray, Exponents};

#[component]
pub fn ALPInfo(array: SharedPtr<ALPArray>) -> Element {
    let Exponents { e, f } = array.exponents();

    rsx! {
        Heading { text: "ALP Encoding Parameters" }

        div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
            table { class: "table-auto w-full min-w-max text-left border-collapse",
                tbody { class: "border-b border-1 border-zinc-50/10",
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Exponent (e)"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{e}"
                            }
                        }
                    }
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Factor (f)"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{f}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// TODO(aduffy): ALP-RD
