// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct NeosProps {
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

pub fn Neos(props: NeosProps) -> Element {
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
        d: "M383.8 512l-95.1 0-108.2-154.5 0 91.1-86.4 63.4-97.7 0 0-482.2 40.5-29.8 108 0 123.7 176.1 0-112.7 86.4-63.4 97.7 0 0 461.5-68.9 50.5zM7.2 35.3l0 460.7 72-52.9 0-249.1 215.5 307.6 84.8 0 52.4-38.2-78.3 0-316.1-450.5-30.2 22.3zM89.7 501.9l80-58.8 0-101-79.8-114.4 0 220.9-72.6 53.3 72.3 0 0 0zM49 10.8l310.6 442.6 82.4 0 0-442.6-79.8 0 0 317.6-222.9-317.6-90.3 0zM279.4 191.6l72 102.8 0-278.5-72 53 0 122.7z",
            }
        }
    }
}
