// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RibbonProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Ribbon(props: RibbonProps) -> Element {
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
        d: "M235.1 0c33.4 0 64.5 17.4 81.9 45.9 1.2 2 13 21.3 35.3 57.8 21.1 34.5 18.3 78.5-7 110L278.3 297.7 364.5 406c5.5 6.9 4.4 16.9-2.5 22.5l-80 64c-6.9 5.5-17 4.4-22.5-2.5L38.6 213.8C13.3 182.3 10.5 138.3 31.6 103.8 54 67.2 65.7 47.9 67 45.9 84.4 17.4 115.4 0 148.9 0l86.3 0zM192 189.2l48.6-61.2-97.3 0 48.6 61.2zM75 336.2l86.2 107.8-36.8 46c-5.5 6.9-15.6 8-22.5 2.5l-80-64c-6.9-5.5-8-15.6-2.5-22.5L75 336.2z",
            }
        }
    }
}
