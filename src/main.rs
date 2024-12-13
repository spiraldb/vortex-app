use std::sync::Arc;

use bytes::Bytes;
use components::{array::ArrayView, AppHeader, ErrorMessage};
use dioxus::prelude::*;
use dioxus::{html::HasFileData, logger::tracing};
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
    let mut file_name = use_signal(|| String::new());
    let mut array_data = use_signal::<Option<SharedArrayData>>(|| None);
    let mut read_error = use_signal::<Option<String>>(|| None);

    // We can provide a writable signal to push a new history element.
    let mut history_stack: Signal<Vec<SharedArrayData>> = use_signal(|| Vec::new());

    let mut dnd_hovered = use_signal(|| false);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        file_name.set(file_engine.files()[0].clone());
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
                    *read_error.write() = None;
                    *array_data.write() = Some(SharedArrayData {
                        inner: Arc::new(array),
                    })
                }
                Err(err) => *read_error.write() = Some(err.to_string()),
            },
        }
    };

    tracing::info!("hello from my element");

    rsx! {
        // Navbar component
        div {
            class: "p-3 flex flex-row items-center gap-x-3",
            class: "border-b border-gray-100/10",
            AppHeader {}
        }

        // Main content
        div { class: "w-full px-4 py-4",

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



            if let Some(error) = read_error() {
                div { class: "my-12 h-0.5 border-t-0 bg-neutral-100/30" }
                ErrorMessage { error }
            } else if let Some(array) = array_data() {
                div { class: "my-12 h-0.5 border-t-0 bg-neutral-100/30" }
                ArrayView { file_name: file_name(), array }
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
