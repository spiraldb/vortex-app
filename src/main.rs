use std::sync::Arc;

use components::{array::ArrayView, Heading};
use dioxus::prelude::*;
use vortex::{
    array::PrimitiveArray, compress::compute_precompression_stats, validity::Validity, ArrayData,
    IntoArrayData,
};

mod components;
mod vortex_file;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LOGO: Asset = asset!("/assets/logo.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Home {}
    }
}

#[component]
fn Home() -> Element {
    // Create a static Vortex array to show its elements here.
    let array = PrimitiveArray::from_vec(vec![1u32, 2, 3], Validity::NonNullable).into_array();
    compute_precompression_stats(&array)?;

    let array = SharedArrayData {
        inner: Arc::new(array),
    };

    rsx! {
        div {
            class: "w-7/12 my-10 mx-auto font-mono p-8 bg-gray-800 rounded-xl border-2 border-gray-500",
            div { class: "flex flex-row items-center",
                img { class: "max-w-8 max-h-8", src: LOGO }
                p { class: "mx-4 font-serif text-2xl", "Vortex File Explorer" }
            }

            ArrayView {
               array: array
            }
        }
    }
}

#[derive(Clone)]
pub struct SharedArrayData {
    inner: Arc<ArrayData>,
}

// impl PartialEq so we can use it as a Prop.
impl PartialEq for SharedArrayData {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}
