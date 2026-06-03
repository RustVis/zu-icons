// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MicrophoneAltProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn MicrophoneAlt(props: MicrophoneAltProps) -> Element {
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
        d: "M96 96c0-53 43-96 96-96 50.3 0 91.6 38.7 95.7 88L232 88c-13.3 0-24 10.7-24 24s10.7 24 24 24l56 0 0 48-56 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l55.7 0c-4.1 49.3-45.3 88-95.7 88-53 0-96-43-96-96L96 96zM24 160c13.3 0 24 10.7 24 24l0 40c0 79.5 64.5 144 144 144s144-64.5 144-144l0-40c0-13.3 10.7-24 24-24s24 10.7 24 24l0 40c0 97.9-73.3 178.7-168 190.5l0 49.5 48 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-144 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l48 0 0-49.5C73.3 402.7 0 321.9 0 224l0-40c0-13.3 10.7-24 24-24z",
            }
        }
    }
}
