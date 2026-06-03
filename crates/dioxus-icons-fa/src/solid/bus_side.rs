// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BusSideProps {
    #[props(default = None)]
    pub title: Option<&'static str>,

    #[props(default = None)]
    pub class: Option<&'static str>,

    #[props(default = None)]
    pub style: Option<&'static str>,

    #[props(default = None)]
    pub width: Option<&'static str>,

    #[props(default = None)]
    pub height: Option<&'static str>,

    #[props(default = None)]
    pub fill: Option<&'static str>,

    #[props(default = None)]
    pub stroke: Option<&'static str>,

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn BusSide(props: BusSideProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            style: props.style,
            height: props.height,
            width: props.width,
            view_box: props.view_box.unwrap_or("0 0 16 16"),
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg"),
            fill: props.fill.unwrap_or("currentColor"),
            stroke: props.stroke,

            if let Some(title_text) = props.title {
                title {{ title_text }}
            }

                        path {
        d: "M480 0c88.4 0 160 71.6 160 160l0 224c0 35.3-28.7 64-64 64l-5.6 0c-13.2 37.3-48.6 64-90.4 64s-77.3-26.7-90.4-64l-139.1 0c-13.2 37.3-48.7 64-90.4 64s-77.2-26.7-90.4-64L64 448c-35.3 0-64-28.7-64-64L0 96C0 43 43 0 96 0L480 0zM160 368a48 48 0 1 0 0 96 48 48 0 1 0 0-96zm320 0a48 48 0 1 0 0 96 48 48 0 1 0 0-96zm0-304c-17.7 0-32 14.3-32 32l0 192c0 17.7 14.3 32 32 32l64 0c17.7 0 32-14.3 32-32l0-128c0-53-43-96-96-96zM248 224l104 0c17.7 0 32-14.3 32-32l0-96c0-17.7-14.3-32-32-32l-104 0 0 160zM96 64C78.3 64 64 78.3 64 96l0 96c0 17.7 14.3 32 32 32l104 0 0-160-104 0z",
            }
        }
    }
}
