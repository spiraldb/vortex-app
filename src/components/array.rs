use std::sync::Arc;

use dioxus::prelude::*;
use vortex::{dtype::DType, stats::{ArrayStatistics, Stat, StatsSet}};

use crate::SharedArrayData;

// Get all of the stats, we iterate this to do lookups
const ALL_STATS: [Stat; 11] = [
    Stat::BitWidthFreq,
    Stat::IsConstant,
    Stat::IsSorted,
    Stat::IsStrictSorted,
    Stat::Max,
    Stat::Min,
    Stat::NullCount,
    Stat::RunCount,
    Stat::TrailingZeroFreq,
    Stat::TrueCount,
    Stat::UncompressedSizeInBytes,
];

/// Show some basic info about an ArrayView.
#[component]
pub fn ArrayView(array: SharedArrayData) -> Element {
    let row_count = array.inner.len();
    let encoding_id = array.inner.encoding().id().to_string();
    let stats = array.inner.statistics().to_set();

    rsx! {
        // schema, row_count

        div {
            // style
            class: "text-sm",

            "Encoding: {encoding_id}"

            p {}

            "Row Count: {row_count}"
        }

        // Stats.
        Statistics { stats: stats }

        // Child array links.
        div {
            class: "text-lg",

            "Children"

        }
    }
}

/// Component to display DType information for an Array.
#[component]
pub fn DTypeInfo(dtype: DType) -> Element {
    // TODO: have a better way of displaying struct types.
    let stringified = format!("{dtype}");
    
    rsx! {
        div {
            class: "text-lg",

            "Schema: {stringified}"
        }
    }
}

#[component]
pub fn Statistics(stats: StatsSet) -> Element {
    rsx! {
        h1 { class: "text-2xl", "Array Statistics" }

        for (stat, value) in stats.clone().into_iter() {
            div {
                class: "font-mono",

                "Statistic - {stat}: {value:?}"
            }
        }
    }
}