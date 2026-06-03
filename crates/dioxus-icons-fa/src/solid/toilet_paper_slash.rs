// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ToiletPaperSlashProps {
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

pub fn ToiletPaperSlash(props: ToiletPaperSlashProps) -> Element {
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
        d: "M41-24.9c-9.4-9.4-24.6-9.4-33.9 0S-2.3-.3 7 9.1l528 528c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-94.5-94.5c40.2-23 69.5-96.9 69.5-184.6 0-106-43-192-96-192s-96 86-96 192c0 24.1 2.2 47.2 6.3 68.4L304 238.2 304 224c0-58.9 11.8-114.5 33.2-157.2 5.9-11.7 13.1-23.7 21.9-34.8L128 32c-8.3 0-16.4 2.1-24.1 6.1l-63-63zM32 224l0 264c0 30.9 25.1 56 56 56l160 0c30.9 0 56-25.1 56-56l0-114.2-256-256C37.9 148.2 32 184.7 32 224zm416 64c-17.7 0-32-28.7-32-64s14.3-64 32-64 32 28.7 32 64-14.3 64-32 64z",
            }
        }
    }
}
