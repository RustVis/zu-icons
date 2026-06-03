// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LaptopHouseProps {
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

pub fn LaptopHouse(props: LaptopHouseProps) -> Element {
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
        d: "M448 240l19.9 0c15.5 0 28.1-12.6 28.1-28.1 0-7.6-3.1-14.9-8.6-20.2L283.5-4.9C276.1-12 266.3-16 256-16s-20.1 4-27.5 11.1L24.6 191.7C19.1 197 16 204.3 16 211.9 16 227.4 28.6 240 44.1 240l19.9 0 0 144c0 35.3 28.7 64 64 64l85.7 0c7.4-6.6 16.4-11.4 26.3-14l0-130c0-5.5 .7-10.9 2-16l-10 0c-13.3 0-24-10.7-24-24l0-48c0-13.3 10.7-24 24-24l48 0c13.3 0 24 10.7 24 24l0 24 144 0zM352 352l160 0 0 128-160 0 0-128zm-64-32l0 160-32 0c-8.8 0-16 7.2-16 16 0 26.5 21.5 48 48 48l288 0c26.5 0 48-21.5 48-48 0-8.8-7.2-16-16-16l-32 0 0-160c0-17.7-14.3-32-32-32l-224 0c-17.7 0-32 14.3-32 32z",
            }
        }
    }
}
