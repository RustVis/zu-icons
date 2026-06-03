// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DiseaseProps {
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

pub fn Disease(props: DiseaseProps) -> Element {
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
        d: "M236.5 29.4L224 48c-20 30-53.6 48-89.7 48L68 96c-37.5 0-67.9 30.4-67.9 67.9 0 18 7.2 35.2 19.9 47.9l27 27c11 11 17.2 25.9 17.2 41.5 0 15.8-6.4 30.9-17.7 42L33.4 335.1c-11.1 10.8-17.3 25.7-17.3 41.2 0 36.8 34.1 64.2 70.1 56.2l62.3-13.8c7.7-1.7 15.7-2.6 23.6-2.6 32.8 0 64.2 14.6 85.2 39.8l30.5 36.6c10.4 12.4 25.7 19.6 41.9 19.6 30.1 0 54.5-24.4 54.5-54.5l0-51.2c0-41.4 25.4-78.5 64-93.5l22.2-8.6c25.2-9.8 41.8-34.1 41.8-61.1 0-26.4-15.9-50.3-40.3-60.5L429 164.8c-33.2-13.9-57.6-43-65.5-78.1l-7.9-35.2c-6.8-30.1-33.5-51.5-64.3-51.5-22 0-42.6 11-54.8 29.4zM160 192a32 32 0 1 1 0 64 32 32 0 1 1 0-64zm96 0a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zm32 96a32 32 0 1 1 0 64 32 32 0 1 1 0-64z",
            }
        }
    }
}
