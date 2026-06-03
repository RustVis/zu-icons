// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GrinTongueProps {
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

pub fn GrinTongue(props: GrinTongueProps) -> Element {
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
        d: "M464 256c0-114.9-93.1-208-208-208S48 141.1 48 256c0 74.1 38.8 139.2 97.1 176-.7-5.2-1.1-10.6-1.1-16l0-53.5c-10.2-12.6-18.3-26.9-23.8-42.4-4.1-11.6 7.8-21.4 19.6-17.8 34.7 10.6 74.2 16.5 116.1 16.5 42 0 81.5-6 116.3-16.6 11.8-3.6 23.7 6.1 19.6 17.8-5.5 15.6-13.6 29.9-23.8 42.5l0 53.5c0 5.4-.4 10.8-1.1 16 58.4-36.8 97.1-101.9 97.1-176zm48 0c0 116.3-77.6 214.6-183.9 245.7-19.5 16.4-44.6 26.3-72.1 26.3s-52.6-9.9-72.1-26.3C77.6 470.6 0 372.3 0 256 0 114.6 114.6 0 256 0S512 114.6 512 256zM176 176a32 32 0 1 1 0 64 32 32 0 1 1 0-64zm128 32a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zm16 208l0-37.4c0-14.7-11.9-26.6-26.6-26.6l-2 0c-11.3 0-21.1 7.9-23.6 18.9-2.8 12.6-20.8 12.6-23.6 0-2.5-11.1-12.3-18.9-23.6-18.9l-2 0c-14.7 0-26.6 11.9-26.6 26.6l0 37.4c0 35.3 28.7 64 64 64s64-28.7 64-64z",
            }
        }
    }
}
