// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HandHoldingUsdProps {
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

pub fn HandHoldingUsd(props: HandHoldingUsdProps) -> Element {
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
        d: "M288-16c-13.3 0-24 10.7-24 24l0 12-1.8 0c-36.6 0-66.2 29.7-66.2 66.2 0 33.4 24.9 61.6 58 65.7l61 7.6c5.1 .6 9 5 9 10.2 0 5.7-4.6 10.2-10.2 10.2L240 180c-15.5 0-28 12.5-28 28s12.5 28 28 28l24 0 0 12c0 13.3 10.7 24 24 24s24-10.7 24-24l0-12 1.8 0c36.6 0 66.2-29.7 66.2-66.2 0-33.4-24.9-61.6-58-65.7l-61-7.6c-5.1-.6-9-5-9-10.2 0-5.7 4.6-10.2 10.2-10.2L328 76c15.5 0 28-12.5 28-28s-12.5-28-28-28l-16 0 0-12c0-13.3-10.7-24-24-24zM109.3 341.5L66.7 384 32 384c-17.7 0-32 14.3-32 32l0 64c0 17.7 14.3 32 32 32l320.5 0c29 0 57.3-9.3 80.7-26.5l126.6-93.3c17.8-13.1 21.6-38.1 8.5-55.9s-38.1-21.6-55.9-8.5L392.6 416 280 416c-13.3 0-24-10.7-24-24s10.7-24 24-24l72 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-152.2 0c-33.9 0-66.5 13.5-90.5 37.5z",
            }
        }
    }
}
