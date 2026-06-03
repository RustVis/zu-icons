// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BoneProps {
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

pub fn Bone(props: BoneProps) -> Element {
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
        d: "M197.4 160c-3.9 0-7.2-2.8-8.1-6.6-10.2-42.1-48.1-73.4-93.3-73.4-53 0-96 43-96 96 0 29.1 12.9 55.1 33.3 72.7 4.3 3.7 4.3 10.8 0 14.5-20.4 17.6-33.3 43.7-33.3 72.7 0 53 43 96 96 96 45.2 0 83.1-31.3 93.3-73.4 .9-3.8 4.2-6.6 8.1-6.6l245.1 0c3.9 0 7.2 2.8 8.1 6.6 10.2 42.1 48.1 73.4 93.3 73.4 53 0 96-43 96-96 0-29.1-12.9-55.1-33.3-72.7-4.3-3.7-4.3-10.8 0-14.5 20.4-17.6 33.3-43.7 33.3-72.7 0-53-43-96-96-96-45.2 0-83.1 31.3-93.3 73.4-.9 3.8-4.2 6.6-8.1 6.6l-245.1 0z",
            }
        }
    }
}
