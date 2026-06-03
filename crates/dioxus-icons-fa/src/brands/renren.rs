// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RenrenProps {
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

pub fn Renren(props: RenrenProps) -> Element {
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
        d: "M214 169.1C214 279.5 153 374.5 66.4 416.5 30 373.2 8 317.7 8 256.6 8 133.9 97.1 32.2 214 12.5l0 156.6zM255 504c-42.9 0-83.3-11-118.5-30.4 57.2-36.1 103.4-90.7 118.5-154.6 15.5 63.9 61.7 118.5 118.8 154.7-35.1 19.3-75.5 30.3-118.8 30.3zm190.6-87.5C359 374.5 298 279.6 298 169.1l0-156.6c116.9 19.7 206 121.4 206 244.1 0 61.1-22 116.6-58.4 159.9z",
            }
        }
    }
}
