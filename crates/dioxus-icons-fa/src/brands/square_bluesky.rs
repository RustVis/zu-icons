// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SquareBlueskyProps {
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

pub fn SquareBluesky(props: SquareBlueskyProps) -> Element {
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
        d: "M64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM224 247.4c14.5-30 54-85.8 90.7-113.3 26.5-19.9 69.3-35.2 69.3 13.7 0 9.8-5.6 82.1-8.9 93.8-11.4 40.8-53 51.2-90 44.9 64.7 11 81.2 47.5 45.6 84-67.5 69.3-97-17.4-104.6-39.6l-.3-.9c-.9-2.6-1.4-4.1-1.8-4.1s-.9 1.5-1.8 4.1c-.1 .3-.2 .6-.3 .9-7.6 22.2-37.1 108.8-104.6 39.6-35.5-36.5-19.1-73 45.6-84-37 6.3-78.6-4.1-90-44.9-3.3-11.7-8.9-84-8.9-93.8 0-48.9 42.9-33.5 69.3-13.7 36.7 27.5 76.2 83.4 90.7 113.3z",
            }
        }
    }
}
