use std::ops::Deref;
use std::sync::Arc;

use crate::components::sample_files::SampleFiles;
use crate::file_util::read_file;
use bytes::Bytes;
use components::{array::ArrayView, AppHeader, ErrorMessage};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus_elements::{FileEngine, HasFileData};
use vortex::{
    file::{LayoutContext, LayoutDeserializer, VortexReadBuilder},
    sampling_compressor::ALL_ENCODINGS_CONTEXT,
    ArrayData,
};

mod components;
mod file_util;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LOGO: Asset = asset!("/assets/logo.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Install a global drag handler so that drag-and-drop opening of files does not
    // bubble up and cause the browser to "download" the file.
    document::eval(
        r#"
        console.log("installing global window event listener for dragover/drop");
        // Have window catch the dragover event.
        window.addEventListener("dragover", function (evt) {
            evt.preventDefault();
        }, false);
        window.addEventListener("drop", function (evt) {
            evt.preventDefault();
        }, false);
        "#,
    );

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Home {}
    }
}

// Starts empty instead.
#[derive(PartialEq, Clone)]
pub struct HistoryStack {
    inner: Vec<HistoryItem>,
}

impl HistoryStack {
    pub fn empty() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, name: String, array: ArrayData) {
        self.inner.push(HistoryItem {
            name,
            array: SharedPtr(Arc::new(array)),
        });
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn goto(&mut self, index: usize) {
        self.inner.truncate(index + 1);
    }

    pub fn current(&self) -> Option<&HistoryItem> {
        self.inner.last()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &HistoryItem> {
        self.inner.iter()
    }
}

#[derive(PartialEq, Clone)]
pub struct HistoryItem {
    pub name: String,
    pub array: SharedPtr<ArrayData>,
}

#[component]
fn Home() -> Element {
    // Create a file reader.
    let mut file_name = use_signal(String::new);
    let mut read_error = use_signal::<Option<String>>(|| None);

    // Push the latest history for each of these elements.
    let mut history_stack: Signal<HistoryStack> = use_signal(HistoryStack::empty);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        let Some(ref file) = files.first() else {
            return;
        };
        file_name.set(file.to_string());
        let contents = file_engine.read_file(&file_engine.files()[0]).await;
        let contents = Bytes::from(contents.unwrap_or_default());

        read_file(file.to_string(), contents, read_error, history_stack).await;
    };

    // True when we have dragged a file but before we drop it.
    let mut dropping = use_signal(|| false);

    rsx! {
        // The entire app is a dropzone.
        div {
            class: "w-full h-screen",
            ondragover: move |_| {
                *dropping.write() = true;
            },
            ondragleave: move |_| {
                *dropping.write() = false;
            },
            ondrop: move |evt| async move {
                evt.prevent_default();
                evt.stop_propagation();
                *dropping.write() = false;
                info!("ondrop event handler called");
                if let Some(file_engine) = evt.files() {
                    info!("files uploaded: {:?}", file_engine.files());
                    read_files(file_engine).await;
                }
            },

            // Navbar component
            div {
                class: "p-3 flex flex-row items-center gap-x-3",
                class: "border-b border-gray-100/10",
                AppHeader {}
            }

            // Main content
            div {
                class: "w-full h-full px-4 py-4",
                class: if dropping() { "border-teal-200 border-double border-2" },

                if history_stack().is_empty() {
                    p { "Drop a Vortex file to view, or upload it below." }

                    input {
                        r#type: "file",
                        accept: ".vortex",
                        multiple: false,
                        onchange: move |evt| async move {
                            *dropping.write() = false;
                            if let Some(file_engine) = evt.files() {
                                read_files(file_engine).await;
                            }
                        }
                    }
                    SampleFiles { file_name, read_error, history_stack }
                } else {
                    if let Some(error) = read_error() {
                        ErrorMessage { error }
                    } else {
                        ArrayView { file_name: file_name(), history_stack }
                    }
                }
            }
        }
    }
}

/// Wrapper around any Arc<T> to make it usable as a Dioxus Prop.
///
/// In Dioxus, all props need must be `PartialEq`. Not all of the Vortex types implement that trait,
/// so this makes it easy for us to pass anything as a component prop at the expense of an added allocation.
#[derive(Clone)]
pub struct SharedPtr<T>(pub Arc<T>);

// Deref impl allowing us to call immutable methods on `T` directly without unwrapping.
impl<T> Deref for SharedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

// Impl PartialEq that ensures two SharedPtr's have the same pointee.
impl<T> PartialEq for SharedPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
