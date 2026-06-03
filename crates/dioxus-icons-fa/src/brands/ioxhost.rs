// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct IoxhostProps {
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

pub fn Ioxhost(props: IoxhostProps) -> Element {
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
        d: "M616 160l-67.3 0C511.2 70.7 422.9 8 320 8 183 8 72 119 72 256 72 272.4 73.6 288.5 76.7 304L24 304c-13.3 0-24 10.8-24 24 0 13.3 10.7 24 24 24l67.3 0c37.5 89.3 125.8 152 228.7 152 137 0 248-111 248-248 0-16.4-1.6-32.5-4.7-48l52.7 0c13.3 0 24-10.8 24-24 0-13.3-10.7-24-24-24zm-96 96c0 110.5-89.5 200-200 200-75.7 0-141.6-42-175.5-104L424 352c13.3 0 24-10.8 24-24 0-13.3-10.7-24-24-24l-298.2 0c-3.8-15.4-5.8-31.4-5.8-48 0-110.5 89.5-200 200-200 75.7 0 141.6 42 175.5 104L216 160c-13.3 0-24 10.8-24 24 0 13.3 10.7 24 24 24l298.2 0c3.8 15.4 5.8 31.4 5.8 48zM216 232l208 0c13.3 0 24 10.7 24 24 0 13.2-10.7 24-24 24l-208 0c-13.3 0-24-10.7-24-24 0-13.2 10.7-24 24-24z",
            }
        }
    }
}
