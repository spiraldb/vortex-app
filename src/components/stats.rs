use dioxus::prelude::*;
use vortex::stats::StatsSet;

use crate::components::Heading;

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
        div { class: "relative flex flex-col max-w-7/12 bg-clip-border",
            table { class: "table-auto w-full min-w-max text-left border-collapse",
                tbody {
                    for (stat , value) in stats.clone().into_iter().map(|(s, v)| (s, v.into_value())) {
                        tr { class: "font-normal hover:bg-neutral-800/75 border-b border-1 border-zinc-50/10",
                            td { class: "p-4",
                                p { class: "block font-sans text-sm antialiased leading-normal",
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
