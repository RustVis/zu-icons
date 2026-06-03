// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HeadSideCoughProps {
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

pub fn HeadSideCough(props: HeadSideCoughProps) -> Element {
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
        d: "M96 512l64 0c53 0 96-43 96-96l0-80c0-8.8 7.2-16 16-16l1 0c26 0 47-21 47-47 0-11-3.9-21.6-10.9-30.1L273.3 200C271.4 197.6 270.1 194.7 269.7 191.7 254 83.3 160.7 0 48 0 40.4 0 32.9 .4 25.4 1.1 10.5 2.6 0 15.9 0 30.9L0 480c0 17.7 14.3 32 32 32l64 0zm16-320a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zm63.1 237.2l-41.5-3.5c-12.2-1-21.6-11.2-21.6-23.4 0-10.8 7.3-20.2 17.8-22.8l40.4-10.1c19.2-4.8 37.8 9.7 37.8 29.5 0 17.8-15.2 31.8-32.9 30.3zM480 312a24 24 0 1 0 0-48 24 24 0 1 0 0 48zm-40 24a24 24 0 1 0 -48 0 24 24 0 1 0 48 0zm-64 48a24 24 0 1 0 -48 0 24 24 0 1 0 48 0zm128 0a24 24 0 1 0 -48 0 24 24 0 1 0 48 0zM480 504a24 24 0 1 0 0-48 24 24 0 1 0 0 48zm-40-72a24 24 0 1 0 -48 0 24 24 0 1 0 48 0z",
            }
        }
    }
}
