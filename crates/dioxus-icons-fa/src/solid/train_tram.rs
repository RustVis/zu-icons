// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TrainTramProps {
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

pub fn TrainTram(props: TrainTramProps) -> Element {
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
        d: "M0 8C0-5.3 10.7-16 24-16l336 0c13.3 0 24 10.7 24 24l0 32c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-8-120 0 0 64 40 0c53 0 96 43 96 96l0 160c0 31.2-14.9 59-38 76.5l64.3 76c8.6 10.1 7.3 25.3-2.8 33.8s-25.3 7.3-33.8-2.8l-74.6-88.1c-3.6 .4-7.3 .6-11.1 .6l-128 0c-3.8 0-7.5-.2-11.1-.6L42.3 535.5c-8.6 10.1-23.7 11.4-33.8 2.8s-11.4-23.7-2.8-33.8l64.3-76C46.9 411 32 383.2 32 352l0-160c0-53 43-96 96-96l40 0 0-64-120 0 0 8c0 13.3-10.7 24-24 24S0 53.3 0 40L0 8zM128 160c-17.7 0-32 14.3-32 32l0 32c0 17.7 14.3 32 32 32l128 0c17.7 0 32-14.3 32-32l0-32c0-17.7-14.3-32-32-32l-128 0zm32 192a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm96 32a32 32 0 1 0 0-64 32 32 0 1 0 0 64z",
            }
        }
    }
}
