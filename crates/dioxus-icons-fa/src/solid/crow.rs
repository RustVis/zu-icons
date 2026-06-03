// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CrowProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Crow(props: CrowProps) -> Element {
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
        d: "M456.5 0c-48.6 0-88 39.4-88 88l0 36-355.2 266.4C-.8 401-3.7 421 6.9 435.2s30.6 17 44.8 6.4l76.8-57.6 131.3 0 46.6 113.1 1 2.2c5.7 10.7 18.8 15.5 30.3 10.8s17.3-17.3 13.9-29l-.8-2.3-39.1-94.9 40.9 0c1.1 0 2.2 0 3.2 0l46.6 113.2 1 2.2c5.7 10.7 18.8 15.5 30.3 10.8s17.3-17.3 13.9-29l-.8-2.3-42-102C485.3 354.1 544.5 280 544.5 192l0-72 80.5-20.1c8.6-2.1 13.8-10.8 11.6-19.4-7.1-28.5-32.7-48.5-62.1-48.5l-50.1 0C508.2 12.5 483.8 0 456.5 0zm0 64a24 24 0 1 1 0 48 24 24 0 1 1 0-48z",
            }
        }
    }
}
