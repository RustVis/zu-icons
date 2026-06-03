// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LegalProps {
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

pub fn Legal(props: LegalProps) -> Element {
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
        d: "M169.6 153.4l-18.7-18.7c-12.5-12.5-12.5-32.8 0-45.3L265.6-25.4c12.5-12.5 32.8-12.5 45.3 0L329.6-6.6c12.5 12.5 12.5 32.8 0 45.3L214.9 153.4c-12.5 12.5-32.8 12.5-45.3 0zM276 211.7l-31.4-31.4 112-112 119.4 119.4-112 112-31.4-31.4-232 232c-15.6 15.6-40.9 15.6-56.6 0s-15.6-40.9 0-56.6l232-232zM390.9 374.6c-12.5-12.5-12.5-32.8 0-45.3L505.6 214.6c12.5-12.5 32.8-12.5 45.3 0l18.7 18.7c12.5 12.5 12.5 32.8 0 45.3L454.9 393.4c-12.5 12.5-32.8 12.5-45.3 0l-18.7-18.7z",
            }
        }
    }
}
