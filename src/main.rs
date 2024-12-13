use std::sync::Arc;

use bytes::Bytes;
use components::{array::ArrayView, AppHeader};
use dioxus::prelude::*;
use dioxus_elements::FileEngine;
use vortex::{
    file::{LayoutContext, LayoutDeserializer, VortexReadBuilder},
    sampling_compressor::ALL_ENCODINGS_CONTEXT,
    ArrayData,
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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Home {}
    }
}

#[component]
fn Home() -> Element {
    // Create a file reader.
    let mut array_data = use_signal::<Option<SharedArrayData>>(|| None);
    let mut read_error = use_signal::<Option<String>>(|| None);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let contents = file_engine.read_file(&file_engine.files()[0]).await;
        let contents = Bytes::from(contents.unwrap_or_default());

        // Create a new VortexFileReader and send the data to it.
        let layout_serde = LayoutDeserializer::new(
            ALL_ENCODINGS_CONTEXT.clone(),
            LayoutContext::default().into(),
        );

        match VortexReadBuilder::new(contents, layout_serde).build().await {
            Err(err) => {
                *read_error.write() = Some(err.to_string());
            }
            Ok(reader) => match reader.read_all().await {
                Ok(array) => {
                    *array_data.write() = Some(SharedArrayData {
                        inner: Arc::new(array),
                    })
                }
                Err(err) => *read_error.write() = Some(err.to_string()),
            },
        }
    };

    rsx! {
        div { class: "w-7/12 my-10 mx-auto p-8 bg-gray-800 rounded-xl border-2 border-gray-500",

            AppHeader {}

            input {
                r#type: "file",
                accept: ".vortex",
                multiple: false,
                onchange: move |evt| async move {
                    if let Some(file_engine) = &evt.files() {
                        read_files(file_engine.clone()).await
                    }
                }
            }


            if let Some(array) = array_data() {
                ArrayView { array }
            }

            if let Some(error) = read_error() {
                pre {
                    "{error}"
                }
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
