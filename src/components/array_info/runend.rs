use dioxus::prelude::*;
use vortex::{compute::try_cast, dtype::PType, runend::RunEndArray, ArrayLen, IntoArrayVariant};

use crate::SharedPtr;

#[component]
pub fn RunEndInfo(array: SharedPtr<RunEndArray>) -> Element {
    let ends = array.ends();
    let ends = try_cast(ends, PType::U64.into())?
        .into_primitive()?
        .into_maybe_null_slice::<u64>();
    let run_lengths: Vec<u64> = ends
        .iter()
        .copied()
        .skip(1)
        .scan(0, |prev, next| {
            let run_length = next - *prev;
            *prev = next;
            Some(run_length)
        })
        .collect();

    let max_run = run_lengths.iter().copied().max().unwrap_or_default();
    let min_run = run_lengths.iter().copied().min().unwrap_or_default();
    let avg_run: f64 = run_lengths
        .iter()
        .copied()
        .map(|run_length| run_length as f64)
        .sum();
    let avg_run = avg_run / (run_lengths.len() as f64);

    rsx! {
        div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
            table { class: "table-auto w-full min-w-max text-left border-collapse",
                tbody { class: "border-b border-1 border-zinc-50/10",
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Run Count"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{run_lengths.len()}"
                            }
                        }
                    }
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Longest Run"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{max_run}"
                            }
                        }
                    }
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Shortest Run"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{min_run}"
                            }
                        }
                    }
                    tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                        td { class: "p-1",
                            p { class: "block font-sans font-bold text-sm antialiased leading-normal",
                                "Average Run Length"
                            }
                        }
                        td { class: "p-1",
                            p { class: "block font-mono text-sm antialiased leading-normal",
                                "{avg_run}"
                            }
                        }
                    }
                }
            }
        }
    }
}
