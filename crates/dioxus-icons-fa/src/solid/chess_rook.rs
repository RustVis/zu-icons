// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ChessRookProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn ChessRook(props: ChessRookProps) -> Element {
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
        d: "M0 32L0 133.5c0 17 6.7 33.3 18.7 45.3L64 224 64 384 7.8 454.3C2.7 460.6 0 468.4 0 476.5 0 496.1 15.9 512 35.5 512l312.9 0c19.6 0 35.5-15.9 35.5-35.5 0-8.1-2.7-15.9-7.8-22.2l-56.2-70.3 0-160 45.3-45.3c12-12 18.7-28.3 18.7-45.3L384 32c0-17.7-14.3-32-32-32L320 0c-17.7 0-32 14.3-32 32l0 32-48 0 0-32c0-17.7-14.3-32-32-32L176 0c-17.7 0-32 14.3-32 32l0 32-48 0 0-32C96 14.3 81.7 0 64 0L32 0C14.3 0 0 14.3 0 32z",
            }
        }
    }
}
