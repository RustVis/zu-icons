// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HatCowboyProps {
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

pub fn HatCowboy(props: HatCowboyProps) -> Element {
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
        d: "M182.2 76.1L130.8 307.5C145.5 324.9 167.4 336 192 336l256 0c24.6 0 46.5-11.1 61.2-28.5L457.8 76.1c-5.7-25.8-28.6-44.1-55-44.1-12.2 0-24.1 4-33.8 11.3l-4.7 3.5c-26.3 19.7-62.4 19.7-88.6 0L271 43.3c-9.8-7.3-21.6-11.3-33.8-11.3-26.4 0-49.3 18.3-55 44.1zM64 256c0-17.7-14.3-32-32-32S0 238.3 0 256C0 362 86 448 192 448l256 0c106 0 192-86 192-192 0-17.7-14.3-32-32-32s-32 14.3-32 32c0 70.7-57.3 128-128 128l-256 0c-70.7 0-128-57.3-128-128z",
            }
        }
    }
}
