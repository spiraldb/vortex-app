use dioxus::prelude::*;

use crate::SharedArrayData;


#[component]
pub fn VortexArray(array: SharedArrayData) -> Element {
    let encoding = array.inner.encoding().id().to_string();
    rsx! {
        "Array with encoding: {encoding}"
    }
}