// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct XmppProps {
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

    #[props(default = Some("0 0 576 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Xmpp(props: XmppProps) -> Element {
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
        d: "M0 47.2c3.5 137.1 117.1 278.6 252.9 356.7-31.5 25-67.2 44.1-106.2 53.9l0 5.4c56.4-2.3 98.1-20.1 141.3-40.7 65.7 34.6 122.2 39.9 141.4 40.7l0-5.4c-39-9.8-74.7-28.8-106.2-53.8 135.7-78.2 249.3-219.8 252.8-356.8-58.4 25.7-119.1 44.2-180.1 62.3l0 0c3.4 59-20.2 170.4-107.9 262.3-88.4-92.3-111.2-204.1-107.8-262.3l0 0C117.7 92.6 59.3 70.2 0 47.2zM93.9 97l73.6 21.8C165.4 216 226.8 358 341.3 432.2 190.5 376.9 89.9 215.7 93.9 97zm388.3 0c3.3 99-65.6 224.7-171.7 296.4-4.7-4.1-9.3-8.4-13.8-12.7 77-80.2 113.2-186.3 111.9-261.9L482.1 97zM264.5 410.5c3 1.6 5.9 3.2 8.9 4.7-12.5 6.4-25.4 12.1-38.6 17 10.3-6.7 20.2-13.9 29.7-21.7z",
            }
        }
    }
}
