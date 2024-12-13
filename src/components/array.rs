use dioxus::prelude::*;
use vortex::{
    dtype::DType,
    stats::{ArrayStatistics, StatsSet},
    validity::ArrayValidity,
    ArrayDType,
};

use crate::{components::Heading, SharedArrayData};

/// Show some basic info about an ArrayView.
#[component]
pub fn ArrayView(array: SharedArrayData) -> Element {
    let stats = array.inner.statistics().to_set();

    rsx! {
        // schema, row_count
        ArraySummary { array: array.clone() }

        // Stats.
        Statistics { stats }

        DTypeInfo { dtype: array.inner.dtype().clone() }
    }
}

#[component]
fn ArraySummary(array: SharedArrayData) -> Element {
    let size = humansize::format_size(array.inner.nbytes(), humansize::BINARY);
    let row_count = array.inner.len();
    let encoding_id = array.inner.encoding().id().to_string();
    let null_count = array.inner.logical_validity().null_count()?;
    let null_pct: f64 = 100. * (null_count as f64) / (row_count as f64);

    rsx! {
        div {
            Heading { text: "Summary" }

            div { class: "relative flex flex-col w-full h-full text-gray-700 font-normal bg-zinc-50 bg-clip-border",
                table { class: "table-auto w-full min-w-max text-left border-collapse",
                    tbody {
                        tr { class: "font-normal text-blue-gray-900 hover:font-bold hover:bg-slate-100 [&:not(:last-child)]:border-b [&:not(:last-child)]:border-blue-gray-50",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Size"
                                }
                            }
                            td { class: "p-4",
                                p { class: "block font-mono text-sm antialiased leading-normal",
                                    "{size}"
                                }
                            }
                        }
                        tr { class: "font-normal text-blue-gray-900 hover:font-bold hover:bg-slate-100 [&:not(:last-child)]:border-b [&:not(:last-child)]:border-blue-gray-50",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Row Count"
                                }
                            }
                            td { class: "p-4",
                                p { class: "block font-mono text-sm antialiased leading-normal",
                                    "{row_count}"
                                }
                            }
                        }
                        tr { class: "font-normal text-blue-gray-900 hover:font-bold hover:bg-slate-100 [&:not(:last-child)]:border-b [&:not(:last-child)]:border-blue-gray-50",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Null Count"
                                }
                            }
                            td { class: "p-4",
                                p { class: "block font-mono text-sm antialiased leading-normal",
                                    "{null_count} ({null_pct}%)"
                                }
                            }
                        }
                        tr { class: "font-normal text-blue-gray-900 hover:font-bold hover:bg-slate-100 [&:not(:last-child)]:border-b [&:not(:last-child)]:border-blue-gray-50",
                            td { class: "p-4",
                                p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                    "Encoding"
                                }
                            }
                            td { class: "p-4",
                                p { class: "block font-mono text-sm antialiased leading-normal",
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
    let stringified = format!("{dtype}");
    // Send a bunch of key/value pairs through the UI

    rsx! {
        Heading { text: "Schema" }
        p {
            "{stringified}"
        }
    }
}

#[component]
pub fn Statistics(stats: StatsSet) -> Element {
    rsx! {
        div {
            Heading { text: "Statistics" }

            div { class: "relative flex flex-col w-full h-full text-gray-700 bg-zinc-50 bg-clip-border",
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
                            tr { class: "font-normal text-blue-gray-900 hover:font-bold hover:bg-slate-100 [&:not(:last-child)]:border-b [&:not(:last-child)]:border-blue-gray-50",
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
}
