use std::sync::Arc;

use components::array::ArrayView;
use dioxus::prelude::*;
use vortex::{array::PrimitiveArray, compress::compute_precompression_stats, stats::ArrayStatistics, validity::Validity, ArrayData, IntoArrayData};
use vortex_file::VortexArray;

mod components;
mod vortex_file;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    // Create a static Vortex array to show its elements here.
    let array = PrimitiveArray::from_vec(vec![1u32, 2, 3], Validity::NonNullable).into_array();
    compute_precompression_stats(&array).unwrap();

    let array = SharedArrayData { 
        inner: Arc::new(array),
    };

    rsx! {
        div { id: "hero", class: "text-2xl",
            "This is my text 2"

            ArrayView {
               array: array 
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
        }

        Outlet::<Route> {}
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
