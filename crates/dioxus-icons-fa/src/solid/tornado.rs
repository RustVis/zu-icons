// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TornadoProps {
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

pub fn Tornado(props: TornadoProps) -> Element {
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
        d: "M0 32L0 45.6C0 62.7 1.7 79.6 5 96l352.8 0c3.2-6.9 7.5-13.3 13-18.8l38.6-38.6c4.2-4.2 6.6-10 6.6-16 0-12.5-10.1-22.6-22.6-22.6L32 0C14.3 0 0 14.3 0 32zM355.7 144l-336 0c12.4 29.7 30.2 56.8 52.7 80l339.6 0-47.2-62.9c-3.9-5.2-7-11-9.2-17.1zM242.6 334.7c8.2 4.8 15.8 10.7 22.5 17.3L445 352c2-9.8 3-19.9 3-30.1 0-17.1-2.9-34-8.6-49.9L136 272 242.6 334.7zM294.3 400c5.3 17 6.5 35.2 3.6 53l-5.9 35.7c-2 12.2 7.4 23.4 19.8 23.4 5.3 0 10.4-2.1 14.2-5.9l78.2-78.2c8.5-8.5 15.8-17.8 21.9-27.9l-131.8 0z",
            }
        }
    }
}
