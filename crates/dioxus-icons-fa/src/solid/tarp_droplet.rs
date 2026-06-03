// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TarpDropletProps {
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

pub fn TarpDroplet(props: TarpDropletProps) -> Element {
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
        d: "M256 160c35.3 0 64-26.9 64-60 0-24-33.7-70.1-52.2-93.5-6.1-7.7-17.5-7.7-23.6 0-18.5 23.4-52.2 69.5-52.2 93.5 0 33.1 28.7 60 64 60zM368 96c0 61.9-50.1 112-112 112S144 157.9 144 96c0-11.1 1.6-21.9 4.6-32L64 64C28.7 64 0 92.7 0 128L0 384c0 35.3 28.7 64 64 64l277.5 0c17 0 33.3-6.7 45.3-18.7L493.3 322.7c12-12 18.7-28.3 18.7-45.3L512 128c0-35.3-28.7-64-64-64l-84.6 0c3 10.1 4.6 20.9 4.6 32zm85.5 176L336 389.5 336 296c0-13.3 10.7-24 24-24l93.5 0zM96 128a32 32 0 1 1 0 64 32 32 0 1 1 0-64z",
            }
        }
    }
}
