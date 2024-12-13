use dioxus::prelude::*;
use vortex::{
    dtype::DType,
    error::vortex_err,
    stats::{ArrayStatistics, StatsSet},
    validity::ArrayValidity,
    ArrayDType,
};

use crate::{components::Heading, SharedArrayData};

/// Show some basic info about an ArrayView.
#[component]
pub fn ArrayView(file_name: String, array: SharedArrayData) -> Element {
    let stats = array.inner.statistics().to_set();

    rsx! {
        // schema, row_count
        ArraySummary { array: array.clone(), file_name: file_name.clone() }

        div { class: "my-12 h-0.5 border-t-0 bg-neutral-100/30" }

        // Stats.
        Statistics { stats }

        div { class: "my-12 h-0.5 border-t-0 bg-neutral-100/30" }

        DTypeInfo { dtype: array.inner.dtype().clone() }

        div { class: "my-12 h-0.5 border-t-0 bg-neutral-100/30" }
    }
}

#[component]
fn ArraySummary(array: SharedArrayData, file_name: String) -> Element {
    let size = humansize::format_size(array.inner.nbytes(), humansize::BINARY);
    let row_count = array.inner.len();
    let encoding_id = array.inner.encoding().id().to_string();
    let null_count = array.inner.logical_validity().null_count()?;
    let null_pct: f64 = 100. * (null_count as f64) / (row_count as f64);

    rsx! {
        div {
            Heading { text: "Summary" }

            div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
                table { class: "table-auto w-full min-w-max text-left border-collapse",
                    tbody { class: "border-b border-1 border-zinc-50/10",
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "File Name"
                                }
                            }
                            td { class: "p-4",
                                p { class: "block font-mono text-sm antialiased leading-normal",
                                    "{file_name}"
                                }
                            }
                        }
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Size"
                                }
                            }
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{size}"
                                }
                            }
                        }
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Row Count"
                                }
                            }
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{row_count}"
                                }
                            }
                        }
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Null Count"
                                }
                            }
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{null_count} ({null_pct}%)"
                                }
                            }
                        }
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Encoding"
                                }
                            }
                            td { class: "p-4",
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
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{field_name}"
                                }
                            }
                            td { class: "p-4",
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

#[component]
pub fn Statistics(stats: StatsSet) -> Element {
    rsx! {
        div {
            Heading { text: "Statistics" }

            if stats.is_empty() {
                p { "No stats" }
            } else {
                StatsTable { stats }
            }
        }
    }
}

#[component]
fn StatsTable(stats: StatsSet) -> Element {
    rsx! {
        div { class: "relative flex flex-col w-full h-full text-gray-700",
            table { class: "table-auto w-full min-w-max text-left border-collapse",
                thead {
                    tr {
                        th { class: "p-4 border-b border-blue-gray-100",
                            p { class: "block font-sans text-sm antialiased font-normal leading-none text-blue-gray-900 opacity-70",
                                "Statistic"
                            }
                        }
                        th { class: "p-4 border-b border-blue-gray-100",
                            p { class: "block font-sans text-sm antialiased font-normal leading-none text-blue-gray-900 opacity-70",
                                "Value"
                            }
                        }
                    }
                }

                tbody {
                    for (stat , value) in stats.clone().into_iter().map(|(s, v)| (s, v.into_value())) {
                        tr { class: "font-normal text-blue-gray-900 hover:font-bold hover:opacity-75 [&:not(:last-child)]:border-b [&:not(:last-child)]:border-blue-gray-50",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "{stat}"
                                }
                            }
                            td { class: "p-4",
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
