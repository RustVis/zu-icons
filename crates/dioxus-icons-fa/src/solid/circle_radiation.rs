// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CircleRadiationProps {
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

pub fn CircleRadiation(props: CircleRadiationProps) -> Element {
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
        d: "M0 256a256 256 0 1 1 512 0 256 256 0 1 1 -512 0zm80 0l64.3 0c8.7 0 15.7-7.1 17.3-15.6 4.4-24.4 18.1-45.5 37.2-59.7 7.4-5.5 10.6-15.6 6-23.6l-32.5-56.3c-4.3-7.5-13.9-10.3-21.2-5.5-48.2 31.5-81.3 84.2-86.3 144.8-.7 8.8 6.5 16 15.3 16zm137.9 89.8c-8.5-3.7-18.8-1.4-23.5 6.6l-31 53.8c-4.3 7.5-1.9 17.2 5.8 21.1 26.1 13.2 55.5 20.7 86.8 20.7s60.7-7.5 86.8-20.7c7.7-3.9 10.1-13.6 5.8-21.1l-31-53.8c-4.6-8-15-10.3-23.5-6.6-11.7 5-24.5 7.8-38.1 7.8s-26.4-2.8-38.1-7.8zM350.4 240.4c1.6 8.6 8.5 15.6 17.3 15.6l64.3 0c8.8 0 16.1-7.2 15.3-16-5-60.6-38.1-113.2-86.3-144.8-7.3-4.8-16.8-2-21.2 5.5L307.3 157c-4.6 8-1.4 18.1 6 23.6 19.1 14.2 32.7 35.4 37.2 59.7zM256 305.7a48 48 0 1 0 0-96 48 48 0 1 0 0 96z",
            }
        }
    }
}
