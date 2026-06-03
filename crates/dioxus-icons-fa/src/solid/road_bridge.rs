// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RoadBridgeProps {
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

pub fn RoadBridge(props: RoadBridgeProps) -> Element {
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
        d: "M32 32l208 0 0 64-24 0 0 64 24 0 0 129.3c-45.4 7.6-80 47.1-80 94.7l0 64c0 17.7-14.3 32-32 32l-32 0c-17.7 0-32-14.3-32-32l0-94c0-38.8-26.4-72.6-64-82l0-112 40 0 0-64-8 0C14.3 96 0 81.7 0 64S14.3 32 32 32zM88 96l0 64 80 0 0-64-80 0zM336 32l72 0 0 72c0 13.3 10.7 24 24 24s24-10.7 24-24l0-72 72 0c26.5 0 48 21.5 48 48l0 352c0 26.5-21.5 48-48 48l-72 0 0-72c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 72-72 0c-26.5 0-48-21.5-48-48l0-352c0-26.5 21.5-48 48-48zm96 160c-13.3 0-24 10.7-24 24l0 80c0 13.3 10.7 24 24 24s24-10.7 24-24l0-80c0-13.3-10.7-24-24-24z",
            }
        }
    }
}
