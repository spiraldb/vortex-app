use std::sync::Arc;

use crate::components::array_info::alp::ALPInfo;
use crate::SharedPtr;
use bitpacked::BitPackedInfo;
use constant::ConstantInfo;
use dict::DictInfo;
use dioxus::prelude::*;
use frame_of_reference::FrameOfReferenceInfo;
use fsst::FSSTInfo;
use runend::RunEndInfo;
use vortex::alp::{ALPArray, ALPEncoding};
use vortex::{
    array::{ConstantArray, ConstantEncoding},
    dict::{DictArray, DictEncoding},
    encoding::Encoding,
    fastlanes::{BitPackedArray, BitPackedEncoding, FoRArray, FoREncoding},
    fsst::{FSSTArray, FSSTEncoding},
    runend::{RunEndArray, RunEndEncoding},
    ArrayData,
};

pub mod alp;
pub mod bitpacked;
pub mod constant;
pub mod dict;
pub mod frame_of_reference;
pub mod fsst;
pub mod runend;

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
    } else if encoding == FoREncoding::ID {
        let array = SharedPtr(Arc::new(FoRArray::try_from(array)?));
        rsx! {
            FrameOfReferenceInfo { array }
        }
    } else if encoding == BitPackedEncoding::ID {
        let array = SharedPtr(Arc::new(BitPackedArray::try_from(array)?));
        rsx! {
            BitPackedInfo { array }
        }
    } else if encoding == RunEndEncoding::ID {
        let array = SharedPtr(Arc::new(RunEndArray::try_from(array)?));
        rsx! {
            RunEndInfo { array }
        }
    } else if encoding == ConstantEncoding::ID {
        let array = SharedPtr(Arc::new(ConstantArray::try_from(array)?));
        rsx! {
            ConstantInfo { array }
        }
    } else if encoding == ALPEncoding::ID {
        let array = SharedPtr(Arc::new(ALPArray::try_from(array)?));
        rsx! {
            ALPInfo { array }
        }
    }
    // Fallback
    else {
        // Empty component
        rsx! {}
    }
}
