// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PaintRollerProps {
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

    #[props(default = Some("0 0 512 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn PaintRoller(props: PaintRollerProps) -> Element {
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
        d: "M0 64C0 28.7 28.7 0 64 0L352 0c35.3 0 64 28.7 64 64l16 0c44.2 0 80 35.8 80 80l0 96c0 44.2-35.8 80-80 80l-160 0c-8.8 0-16 7.2-16 16l0 18.7c18.6 6.6 32 24.4 32 45.3l0 96c0 26.5-21.5 48-48 48l-32 0c-26.5 0-48-21.5-48-48l0-96c0-20.9 13.4-38.7 32-45.3l0-18.7c0-44.2 35.8-80 80-80l160 0c8.8 0 16-7.2 16-16l0-96c0-8.8-7.2-16-16-16l-16 0c0 35.3-28.7 64-64 64L64 192c-35.3 0-64-28.7-64-64L0 64z",
            }
        }
    }
}
