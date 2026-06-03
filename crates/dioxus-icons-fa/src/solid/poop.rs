// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PoopProps {
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

pub fn Poop(props: PoopProps) -> Element {
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
        d: "M254.4 6.6c3.5-4.3 9-6.5 14.5-5.7 46.9 6.3 83.1 46.5 83.1 95.1 0 11.2-1.9 22-5.5 32l5.5 0c35.3 0 64 28.7 64 64 0 19.1-8.4 36.3-21.7 48l13.7 0c39.8 0 72 32.2 72 72 0 23.2-11 43.8-28 57 34.1 5.7 60 35.3 60 71 0 39.8-32.2 72-72 72L72 512c-39.8 0-72-32.2-72-72 0-35.7 25.9-65.3 60-71-17-13.2-28-33.8-28-57 0-39.8 32.2-72 72-72l13.7 0c-13.3-11.7-21.7-28.9-21.7-48 0-35.3 28.7-64 64-64l16.2 0c44.1-.1 79.8-35.9 79.8-80 0-9.2-1.5-17.9-4.3-26.1-1.8-5.2-.8-11.1 2.8-15.4z",
            }
        }
    }
}
