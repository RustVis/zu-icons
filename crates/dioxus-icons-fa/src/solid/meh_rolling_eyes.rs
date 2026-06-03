// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MehRollingEyesProps {
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

pub fn MehRollingEyes(props: MehRollingEyesProps) -> Element {
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
        d: "M256 512a256 256 0 1 0 0-512 256 256 0 1 0 0 512zM192 352l128 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-128 0c-13.3 0-24-10.7-24-24s10.7-24 24-24zm32-128c0 35.3-28.7 64-64 64s-64-28.7-64-64c0-26 15.5-48.4 37.8-58.4-3.7 5.2-5.8 11.6-5.8 18.4 0 17.7 14.3 32 32 32s32-14.3 32-32c0-6.9-2.2-13.2-5.8-18.4 22.3 10 37.8 32.4 37.8 58.4zm128 64c-35.3 0-64-28.7-64-64 0-26 15.5-48.4 37.8-58.4-3.7 5.2-5.8 11.6-5.8 18.4 0 17.7 14.3 32 32 32s32-14.3 32-32c0-6.9-2.2-13.2-5.8-18.4 22.3 10 37.8 32.4 37.8 58.4 0 35.3-28.7 64-64 64z",
            }
        }
    }
}
