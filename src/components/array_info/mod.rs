use std::sync::Arc;

use dict::DictInfo;
use dioxus::prelude::*;
use fsst::FSSTInfo;
use vortex::{
    dict::{DictArray, DictEncoding},
    encoding::Encoding,
    fsst::{FSSTArray, FSSTEncoding},
    ArrayData,
};

use crate::SharedPtr;

pub mod dict;
pub mod fsst;

/// Show encoding-specific information about an array.
///
/// This is a parent component that will dynamically delegate to the encoding-specific child component.
#[component]
pub fn EncodingInfo(array: SharedPtr<ArrayData>) -> Element {
    let array = (*array).clone();
    let encoding = array.encoding().id();

    if encoding == FSSTEncoding::ID {
        // Show FSST symbol table info.
        let array = SharedPtr(Arc::new(FSSTArray::try_from(array)?));
        rsx! {
            FSSTInfo { array }
        }
    } else if encoding == DictEncoding::ID {
        // Show dictionary size, value sample, histogram
        let array = SharedPtr(Arc::new(DictArray::try_from(array)?));
        rsx! {
            DictInfo { array }
        }
    } else {
        // Empty component
        rsx! {}
    }
}
