#![allow(unused)]

use dioxus::prelude::*;

// TODO(aduffy): Convert all of our tables to types in here instead.
//  Need to figure out how Dioxus' component model works if you have multiple children components.

#[component]
pub fn TableHeader(names: Vec<String>) -> Element {
    rsx! {
        thead {
            tr {
                for header in names {
                    th { class: "p-4 border-b border-blue-gray-100",
                        p { class: "block font-sans text-sm antialiased font-normal leading-none text-blue-gray-900 opacity-70",
                            "{header}"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum FontStyle {
    #[default]
    Inherit,
    Mono,
    Sans,
    Serif,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum FontWeight {
    #[default]
    Inherit,
    Normal,
    Bold,
}

#[derive(PartialEq, Props, Clone)]
pub struct TableCellProps {
    #[props(default)]
    font_family: FontStyle,
    #[props(default)]
    font_weight: FontWeight,
    text: String,
}

/// Let's create a few of these cell values instead.
#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    let font_family = match props.font_family {
        FontStyle::Inherit => "",
        FontStyle::Mono => "font-mono",
        FontStyle::Sans => "font-sans",
        FontStyle::Serif => "font-serif",
    };

    let font_weight = match props.font_weight {
        FontWeight::Inherit => "",
        FontWeight::Normal => "font-normal",
        FontWeight::Bold => "font-bold",
    };

    rsx! {
        td { class: "p-4",
            p {
                class: "block text-sm antialiased leading-normal",
                class:  "{font_family}",
                class:  "{font_weight}",
                "{props.text}"
            }
        }
    }
}
