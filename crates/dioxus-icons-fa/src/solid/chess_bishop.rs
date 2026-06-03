// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ChessBishopProps {
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

    #[props(default = Some("0 0 320 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn ChessBishop(props: ChessBishopProps) -> Element {
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
        d: "M64 384L48.3 368.3C17.4 337.4 0 295.4 0 251.7 0 213.1 13.5 175.8 38.2 146.1L106.7 64 96 64C78.3 64 64 49.7 64 32S78.3 0 96 0L224 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l-10.7 0 47.6 57.1-85.9 85.9c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0l82.3-82.3c18.7 27.3 28.7 59.7 28.7 93 0 43.7-17.4 85.7-48.3 116.6L256 384 312.2 454.3c5 6.3 7.8 14.1 7.8 22.2 0 19.6-15.9 35.5-35.5 35.5L35.5 512c-19.6 0-35.5-15.9-35.5-35.5 0-8.1 2.7-15.9 7.8-22.2L64 384z",
            }
        }
    }
}
