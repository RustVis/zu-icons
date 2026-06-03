// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GrinBeamSweatProps {
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

pub fn GrinBeamSweat(props: GrinBeamSweatProps) -> Element {
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
        d: "M546.2-.1c14.7 17.8 29.8 40.1 29.8 64.1 0 36.4-27.6 64-64 64s-64-27.6-64-64c0-24 15.2-46.3 29.8-64.1 8.8-10.7 18.5-20.8 29-30 3-2.6 7.4-2.6 10.4 0 10.5 9.1 20.1 19.3 29 30zM288 0c42.5 0 82.6 10.4 117.8 28.7-3.6 10.9-5.8 22.7-5.8 35.3 0 62.9 49.1 112 112 112 6.4 0 12.7-.5 18.8-1.5 8.6 25.6 13.2 53 13.2 81.5 0 141.4-114.6 256-256 256S32 397.4 32 256 146.6 0 288 0zM418.7 308.9C379.5 321.1 335.1 328 288 328s-91.5-6.9-130.7-19.1c-11.9-3.7-23.9 6.3-19.6 18.1 22.4 61.3 81.3 105.1 150.3 105.1s127.9-43.8 150.3-105.1c4.3-11.8-7.7-21.8-19.6-18.1zM208 180c15.5 0 28 12.5 28 28l0 8c0 11 9 20 20 20s20-9 20-20l0-8c0-37.6-30.4-68-68-68s-68 30.4-68 68l0 8c0 11 9 20 20 20s20-9 20-20l0-8c0-15.5 12.5-28 28-28zm132 28c0-15.5 12.5-28 28-28s28 12.5 28 28l0 8c0 11 9 20 20 20s20-9 20-20l0-8c0-37.6-30.4-68-68-68s-68 30.4-68 68l0 8c0 11 9 20 20 20s20-9 20-20l0-8z",
            }
        }
    }
}
