use dioxus::prelude::*;

use crate::HistoryStack;
use bytes::Bytes;
use vortex::buffer::Buffer;
use vortex::file::{LayoutContext, LayoutDeserializer, VortexReadBuilder};
use vortex::sampling_compressor::ALL_ENCODINGS_CONTEXT;

pub async fn read_file(
    file: String,
    contents: Bytes,
    mut read_error: Signal<Option<String>>,
    mut history_stack: Signal<HistoryStack>,
) {
    let contents = Buffer::from(contents);
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
                // Push onto the front of the stack.
                history_stack.write().clear();
                history_stack.write().push(file, array);
            }
            Err(err) => *read_error.write() = Some(err.to_string()),
        },
    }
}
