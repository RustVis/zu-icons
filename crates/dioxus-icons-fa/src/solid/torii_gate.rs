// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ToriiGateProps {
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

    #[props(default = Some("0 0 448 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn ToriiGate(props: ToriiGateProps) -> Element {
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
        d: "M0 96L0 28.5C0 21.6 5.6 16 12.5 16 14.8 16 17 16.6 19 17.8L58 41.2C82.9 56.1 111.3 64 140.3 64l167.4 0c29 0 57.5-7.9 82.3-22.8l39-23.4c1.9-1.2 4.2-1.8 6.4-1.8 6.9 0 12.5 5.6 12.5 12.5L448 96c0 35.3-28.7 64-64 64l0 64 16 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l-16 0 0 192c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-192-192 0 0 192c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-192-16 0c-17.7 0-32-14.3-32-32s14.3-32 32-32l16 0 0-64C28.7 160 0 131.3 0 96zM128 224l64 0 0-64-64 0 0 64zm128 0l64 0 0-64-64 0 0 64z",
            }
        }
    }
}
