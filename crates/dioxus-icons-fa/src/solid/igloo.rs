// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct IglooProps {
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

pub fn Igloo(props: IglooProps) -> Element {
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
        d: "M320 33.8l0 126.2-271.5 0c51.7-77.2 139.6-128 239.5-128 10.8 0 21.5 .6 32 1.8zM368 160l0-116.7c66 19 122.2 61 159.5 116.7L368 160zM22.6 208l73.4 0 0 112-96 0c0-39.7 8-77.6 22.6-112zM176 320l-32 0 0-112 288 0 0 112-32 0 0 48 176 0 0 64c0 26.5-21.5 48-48 48L48 480c-26.5 0-48-21.5-48-48l0-64 176 0 0-48zm304 0l0-112 73.4 0C568 242.4 576 280.3 576 320l-96 0zM288 288c-35.3 0-64 28.7-64 64l0 80 128 0 0-80c0-35.3-28.7-64-64-64z",
            }
        }
    }
}
