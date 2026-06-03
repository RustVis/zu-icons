// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct Typo3Props {
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

pub fn Typo3(props: Typo3Props) -> Element {
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
        d: "M178.7 78.4c0-24.7 5.4-32.4 13.9-39.4-69.5 8.5-149.3 34-176.3 66.4-5.4 7.7-9.3 20.8-9.3 37.1 0 103.5 106.8 337.5 184.1 337.5 36.3 0 97.3-59.5 146.7-139-7 2.3-11.6 2.3-18.5 2.3-57.2 0-140.6-198.5-140.6-264.9zM301.5 32c-30.1 0-41.7 5.4-41.7 36.3 0 66.4 53.8 198.5 101.7 198.5 26.3 0 78.8-99.7 78.8-182.3 0-40.9-67-52.5-138.8-52.5z",
            }
        }
    }
}
