use crate::{HistoryItem, HistoryStack};
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::HiChevronRight;
use dioxus_free_icons::Icon;

#[component]
pub fn Breadcrumbs(history_stack: Signal<HistoryStack>) -> Element {
    let last_idx = history_stack().len().checked_sub(1).unwrap_or_default();
    rsx! {
        div { class: "flex flex-row items-center overflow-x-auto",
            if let Some(HistoryItem { name, .. }) = history_stack().iter().next() {
                p {
                    class: "text-md font-bold",
                    class: if last_idx > 0 { "hover:text-sky-500 cursor-pointer" },

                    onclick: move |_| {
                        if last_idx > 0 {
                            history_stack.write().goto(0);
                        }
                    },
                    "{name}"
                }
            }
            for (idx , item) in history_stack().iter().enumerate().skip(1) {
                div {
                    Icon { width: 30, height: 30, icon: HiChevronRight }
                }

                // Make it clickable to revert back to this element in the tree instead.
                p {
                    class: "text-md font-bold",
                    class: if idx != last_idx { "hover:text-sky-500 cursor-pointer" },
                    onclick: move |_| {
                        if idx != last_idx {
                            history_stack.write().goto(idx);
                        }
                    },

                    "{item.name}"
                }
            }
        }
    }
}
