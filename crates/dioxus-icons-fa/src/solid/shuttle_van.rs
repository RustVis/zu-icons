// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ShuttleVanProps {
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

pub fn ShuttleVan(props: ShuttleVanProps) -> Element {
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
        d: "M64 64C28.7 64 0 92.7 0 128L0 336c0 35.3 28.7 64 64 64l.4 0c4 44.9 41.7 80 87.6 80s83.6-35.1 87.6-80l104.7 0c4 44.9 41.7 80 87.6 80 46.1 0 83.9-35.4 87.7-80.5 31.7-3.8 56.3-30.8 56.3-63.5l0-101.3c0-13.8-4.5-27.3-12.8-38.4l-80-106.7C471.1 73.5 452.1 64 432 64L64 64zM504 224l-120 0 0-96 48 0 72 96zM64 224l0-96 96 0 0 96-96 0zm160 0l0-96 96 0 0 96-96 0zM392 392a40 40 0 1 1 80 0 40 40 0 1 1 -80 0zM152 352a40 40 0 1 1 0 80 40 40 0 1 1 0-80z",
            }
        }
    }
}
