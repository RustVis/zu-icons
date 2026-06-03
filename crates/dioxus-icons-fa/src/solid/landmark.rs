// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LandmarkProps {
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

pub fn Landmark(props: LandmarkProps) -> Element {
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
        d: "M238.7 5.1c10.5-6.8 24.1-6.8 34.6 0l224 144c11.9 7.7 17.4 22.3 13.4 35.9s-16.5 23-30.7 23l-32 0 0 208 51.2 38.4c8.1 6 12.8 15.5 12.8 25.6 0 17.7-14.3 32-32 32L32 512c-17.7 0-32-14.3-32-32 0-10.1 4.7-19.6 12.8-25.6l51.2-38.4 0 0 0-208-32 0c-14.2 0-26.7-9.4-30.7-23s1.5-28.3 13.4-35.9l224-144zM336 208l0 208 64 0 0-208-64 0zM224 416l64 0 0-208-64 0 0 208zM112 208l0 208 64 0 0-208-64 0z",
            }
        }
    }
}
