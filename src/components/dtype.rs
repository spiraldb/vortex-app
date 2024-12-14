use dioxus::prelude::*;
use vortex::{dtype::DType, error::vortex_err};

use crate::components::Heading;

/// Component to display DType information for an Array.
#[component]
pub fn DTypeInfo(dtype: DType) -> Element {
    let inner = if dtype.is_struct() {
        rsx! {
            SchemaTable { dtype }
        }
    } else {
        let stringified = format!("{dtype}");
        rsx! {
            p { "{stringified}" }
        }
    };

    rsx! {
        Heading { text: "Schema" }

        {inner}
    }
}

#[component]
pub fn SchemaTable(dtype: DType) -> Element {
    let field_names = dtype
        .as_struct()
        .ok_or(vortex_err!("SchemaTable must receive a StructDType"))?
        .names();
    let field_types = dtype
        .as_struct()
        .ok_or(vortex_err!("SchemaTable must receive a StructDType"))?
        .dtypes();
    let names_and_types = field_names.iter().zip(field_types.iter());

    rsx! {
        div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
            table { class: "table-auto w-full min-w-max text-left border-collapse",
                thead {
                    class: "bg-neutral-700 border-b border-1 border-zinc-50/10",
                    tr {
                        th { class: "p-4",
                            p { class: "block font-sans text-sm antialiased font-normal leading-none opacity-70",
                                "Field Name"
                            }
                        }
                        th { class: "p-4",
                            p { class: "block font-sans text-sm antialiased font-normal leading-none opacity-70",
                                "Type"
                            }
                        }
                    }
                }

                tbody {
                    for (field_name, field_type) in names_and_types {
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-1",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{field_name}"
                                }
                            }
                            td { class: "p-1",
                                p { class: "block font-mono text-sm antialiased leading-normal",
                                    "{field_type}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
