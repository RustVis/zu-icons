// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PepperHotProps {
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

pub fn PepperHot(props: PepperHotProps) -> Element {
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
        d: "M545 65c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0L478.9 63.2C452.3 43.6 419.5 32 384 32 348.9 32 316.5 43.3 290.1 62.4 274.1 74.1 284.2 96 304 96l24 0c13.3 0 24 10.7 24 24l0 80c0 13.3 10.7 24 24 24l80 0c13.3 0 24 10.7 24 24l0 24c0 19.8 21.9 29.9 33.6 13.9 19.1-26.4 30.4-58.8 30.4-93.9 0-35.5-11.6-68.3-31.2-94.9L545 65zM270 135.5l-159.6 228C101.3 376.4 86.7 384 71 384l-7 0c-26.5 0-48 21.5-48 48s21.5 48 48 48l27.1 0c65.7 0 129.7-20.2 183.5-57.8L440.5 306.1C435 295.9 432 284.2 432 272l-56 0c-39.8 0-72-32.2-72-72l0-56c-12.2 0-23.9-3-34-8.5z",
            }
        }
    }
}
