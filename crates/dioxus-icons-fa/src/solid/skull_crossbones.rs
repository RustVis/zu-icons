// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SkullCrossbonesProps {
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

pub fn SkullCrossbones(props: SkullCrossbonesProps) -> Element {
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
        d: "M384 144C384 64.5 312.4 0 224 0S64 64.5 64 144c0 47.1 25.1 88.9 64 115.2l0 28.8c0 17.7 14.3 32 32 32l128 0c17.7 0 32-14.3 32-32l0-28.8c38.9-26.3 64-68.1 64-115.2zM160 128a32 32 0 1 1 0 64 32 32 0 1 1 0-64zm96 32a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zM445.5 339.7c-6.8-16.3-25.5-24-41.8-17.2L224 397.3 44.3 322.5c-16.3-6.8-35 .9-41.8 17.2s.9 35 17.2 41.8L140.8 432 19.7 482.5C3.4 489.3-4.3 508 2.5 524.3s25.5 24 41.8 17.2L224 466.7 403.7 541.5c16.3 6.8 35-.9 41.8-17.2s-.9-35-17.2-41.8L307.2 432 428.3 381.5c16.3-6.8 24-25.5 17.2-41.8z",
            }
        }
    }
}
