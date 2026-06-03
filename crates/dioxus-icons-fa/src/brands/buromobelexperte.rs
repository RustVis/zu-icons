// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BuromobelexperteProps {
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

pub fn Buromobelexperte(props: BuromobelexperteProps) -> Element {
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
        d: "M0 32l0 128 128 0 0-128-128 0zM120 152l-112 0 0-112 112 0 0 112zM160 32l0 128 128 0 0-128-128 0zM280 152l-112 0 0-112 112 0 0 112zM320 32l0 128 128 0 0-128-128 0zM440 152l-112 0 0-112 112 0 0 112zM0 192l0 128 128 0 0-128-128 0zM120 312l-112 0 0-112 112 0 0 112zm40-120l0 128 128 0 0-128-128 0zM280 312l-112 0 0-112 112 0 0 112zm40-120l0 128 128 0 0-128-128 0zM440 312l-112 0 0-112 112 0 0 112zM0 352l0 128 128 0 0-128-128 0zM120 472l-112 0 0-112 112 0 0 112zm40-120l0 128 128 0 0-128-128 0zM280 472l-112 0 0-112 112 0 0 112zm40-120l0 128 128 0 0-128-128 0z",
            }
        }
    }
}
