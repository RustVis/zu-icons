// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PersonCaneProps {
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

pub fn PersonCane(props: PersonCaneProps) -> Element {
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
        d: "M232-32a56 56 0 1 1 0 112 56 56 0 1 1 0-112zM144 236.7L97.7 299.1c-10.5 14.2-30.6 17.2-44.8 6.6s-17.2-30.6-6.6-44.8l70.5-95C142 132 181.7 112 224 112s82 20 107.2 53.9l70.5 95c10.5 14.2 7.6 34.2-6.6 44.8s-34.2 7.6-44.8-6.6L304 236.7 304 512c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-160c0-8.8-7.2-16-16-16s-16 7.2-16 16l0 160c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-275.3zM392 384c-4.4 0-8 3.6-8 8 0 13.3-10.7 24-24 24s-24-10.7-24-24c0-30.9 25.1-56 56-56s56 25.1 56 56l0 128c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-128c0-4.4-3.6-8-8-8z",
            }
        }
    }
}
