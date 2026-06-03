// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CandyCaneProps {
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

pub fn CandyCane(props: CandyCaneProps) -> Element {
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
        d: "M421.8 39.6C452 85.2 456.5 142.9 435.1 192l-165.4 0 41.4-41.4c.8-.8 1.5-1.6 2.2-2.4L421.8 39.6zM268 102.9c-.8 .7-1.6 1.4-2.4 2.2l-28.1 28.1c-25 25-65.5 25-90.5 0s-25-65.5 0-90.5L175 14.6C229.9-40.2 314.5-46.7 376.5-5.6L268 102.9zM125.7 336l80-80 181 0-82.5 82.5 0-2.5-178.5 0zm117 64L141.4 501.3c-25 25-65.5 25-90.5 0s-25-65.5 0-90.5l10.7-10.7 181 0z",
            }
        }
    }
}
