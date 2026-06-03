// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UserInjuredProps {
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

pub fn UserInjured(props: UserInjuredProps) -> Element {
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
        d: "M242.7 80L334 80c-10.7-24.6-29.5-44.9-52.9-57.6L242.7 80zm-.9-70.7C236 8.4 230 8 224 8 174.8 8 132.5 37.6 114 80l80.6 0 47.1-70.7zM224 248c66.3 0 120-53.7 120-120l-240 0c0 66.3 53.7 120 120 120zM98.7 341.8C49.3 370.2 16 423.5 16 484.6 16 499.7 28.3 512 43.4 512l151 0-95.8-170.2zm45.1-17.7l42.7 75.9 85.5 0c44.2 0 80 35.8 80 80 0 11.4-2.4 22.2-6.7 32l59.2 0c15.1 0 27.4-12.3 27.4-27.4 0-90.9-73.7-164.6-164.6-164.6l-86.9 0c-12.6 0-24.9 1.4-36.7 4.1zM213.5 448l36 64 22.5 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-58.5 0z",
            }
        }
    }
}
