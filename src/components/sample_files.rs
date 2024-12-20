use crate::file_util::read_file;
use crate::HistoryStack;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

/// Set of pre-created sample files to let users play around with Vortex.
///
/// Entries are tuples of (file_name, file_url).
const SAMPLE_FILES: [(&str, &str); 1] = [
    // The linteitem TPC-H table.
    (
        "supplier.vortex",
        "https://vortex-public-sample-files.s3.amazonaws.com/supplier.vortex",
    ),
];

#[derive(Debug, PartialEq, Eq)]
enum LoadState {
    Default,
    Loading,
    Finished,
}

#[component]
pub fn SampleFiles(
    mut file_name: Signal<String>,
    mut read_error: Signal<Option<String>>,
    mut history_stack: Signal<HistoryStack>,
) -> Element {
    let mut load_state = use_signal(|| LoadState::Loading);

    let read_sample_file = move |file: String, file_url: String| async move {
        info!("fetching {file_url}");
        let contents = reqwest::Client::new()
            .get(file_url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        info!("completed request to server for demo file");
        *file_name.write() = file.clone();
        read_file(file, contents, read_error, history_stack).await;
        *load_state.write() = LoadState::Finished;
    };

    rsx! {
        div { class: "p-4",
            p { "Or, select a sample file below:" }
            ul {
                for (file_name , file_url) in SAMPLE_FILES.iter().cloned() {
                    li {
                        a {
                            class: "cursor-pointer text-sky-500 text-bold",
                            class: if *load_state.read() == LoadState::Loading { "" },
                            onclick: move |_| async move {
                                *load_state.write() = LoadState::Loading;
                                info!("clicked on {file_url}");
                                read_sample_file(file_name.to_string(), file_url.to_string()).await;
                            },
                            "{file_name}"
                        }
                    }
                }
            }
        }
    }
}
