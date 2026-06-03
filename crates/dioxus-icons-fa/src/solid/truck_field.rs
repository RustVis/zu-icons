// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TruckFieldProps {
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

pub fn TruckField(props: TruckFieldProps) -> Element {
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
        d: "M32 96c0-35.3 28.7-64 64-64l224 0c23.7 0 44.4 12.9 55.4 32l40.4 0c32.4 0 61.6 19.6 74 49.5l43.8 106.3 1.7 4.2 24.7 0c26.5 0 48 21.5 48 48l0 64c0 26.5-21.5 48-48 48l-.4 0c.2 2.6 .4 5.3 .4 8 0 48.6-39.4 88-88 88s-88-39.4-88-88c0-2.7 .1-5.4 .4-8l-144.7 0c.2 2.6 .4 5.3 .4 8 0 48.6-39.4 88-88 88s-88-39.4-88-88c0-3.6 .2-7.1 .6-10.5-19-6.4-32.6-24.4-32.6-45.5l0-48c-17.7 0-32-14.3-32-32l0-96c0-17.7 14.3-32 32-32l0-32zm352 32l0 96 82.1 0-35.4-86.1c-2.5-6-8.3-9.9-14.8-9.9L384 128zM192 392a40 40 0 1 0 -80 0 40 40 0 1 0 80 0zm280 40a40 40 0 1 0 0-80 40 40 0 1 0 0 80z",
            }
        }
    }
}
