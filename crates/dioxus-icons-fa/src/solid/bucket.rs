// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BucketProps {
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

pub fn Bucket(props: BucketProps) -> Element {
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
        d: "M443.7 208c2.7 4.7 4.3 10.2 4.3 16 0 17.7-14.3 32-32 32l-5.1 0-22.4 213c-2.6 24.4-23.2 43-47.8 43l-233.6 0c-24.6 0-45.2-18.5-47.8-43L37.1 256 32 256c-17.7 0-32-14.3-32-32 0-5.8 1.6-11.3 4.3-16l439.4 0zM224-16c79.5 0 144 64.5 144 144l0 32-48 0 0-32c0-53-43-96-96-96s-96 43-96 96l0 32-48 0 0-32C80 48.5 144.5-16 224-16z",
            }
        }
    }
}
