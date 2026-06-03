// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CruzeiroSignProps {
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

pub fn CruzeiroSign(props: CruzeiroSignProps) -> Element {
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
        d: "M240 96c-88.4 0-160 71.6-160 160 0 71.6 47.1 132.3 112 152.7L192 256c0-13.3 10.7-24 24-24 12.8 0 23.3 10.1 24 22.7 15.5-9.5 33.5-14.7 52.3-14.7l11.8 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-11.8 0c-18.8 0-36.1 10-45.4 26.3-4.5 7.9-6.9 16.8-6.9 25.9l0 75.8c42.5 0 81.1-16.6 109.8-43.6 12.9-12.1 33.1-11.5 45.2 1.3s11.5 33.1-1.3 45.2C353.7 456.8 299.5 480 240 480 116.3 480 16 379.7 16 256S116.3 32 240 32c59.5 0 113.7 23.2 153.7 61.1 12.9 12.1 13.4 32.4 1.3 45.2s-32.4 13.4-45.2 1.3C321.1 112.6 282.5 96 240 96z",
            }
        }
    }
}
