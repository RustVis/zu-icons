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
        d: "M352 0c17.7 0 32 14.3 32 32l0 138.7c0 13.8-4.5 27.3-12.8 38.4l-35.2 46.9 0 112 40.8 68.1c4.7 7.8 7.2 16.7 7.2 25.8 0 27.7-22.4 50.1-50.1 50.1L50.1 512c-27.7 0-50.1-22.4-50.1-50.1 0-9.1 2.5-18 7.2-25.8L48 368 48 256 12.8 209.1C4.5 198 0 184.5 0 170.7L0 32C0 14.3 14.3 0 32 0L352 0zM48.3 460.8l-.3 1.1c0 1.2 1 2.1 2.1 2.1l283.8 0c1.2 0 2.1-1 2.1-2.1l-.3-1.1-36.5-60.8-214.4 0-36.5 60.8zM48 170.7c0 2.6 .6 5.1 1.8 7.4l1.4 2.2 0 0 35.2 46.9 9.6 12.8 0 112 192 0 0-112 9.6-12.8 35.2-46.9 0 0 1.4-2.2c1.2-2.3 1.8-4.8 1.8-7.4l0-122.7-64 0 0 24c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-24-64 0 0 24c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-24-64 0 0 122.7z",
            }
        }
    }
}
