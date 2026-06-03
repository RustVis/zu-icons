// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ShoePrintsProps {
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

pub fn ShoePrints(props: ShoePrintsProps) -> Element {
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
        d: "M296 192c-21.1-12.1-42.3-24.2-72-29.3l0-140.3C257.7 13 311.4 0 352 0 448 0 576 48 576 128s-119.6 96-176 96c-48 0-76-16-104-32zM128 32l48 0 0 128-48 0c-35.3 0-64-28.7-64-64s28.7-64 64-64zM232 320c28-16 56-32 104-32 56.4 0 176 16 176 96S384 512 288 512c-40.5 0-94.3-13-128-22.4l0-140.3c29.7-5.2 50.9-17.3 72-29.4zM64 480c-35.3 0-64-28.7-64-64s28.7-64 64-64l48 0 0 128-48 0z",
            }
        }
    }
}
