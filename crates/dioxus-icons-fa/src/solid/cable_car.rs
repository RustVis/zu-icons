// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CableCarProps {
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

pub fn CableCar(props: CableCarProps) -> Element {
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
        d: "M288-32a32 32 0 1 1 0 64 32 32 0 1 1 0-64zM160 24a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zM32 288c0-35.3 28.7-64 64-64l136 0 0-96.2-193.4 55.3c-12.7 3.6-26-3.7-29.7-16.5s3.7-26 16.5-29.7l224-64 0 0 224-64c12.7-3.6 26 3.7 29.7 16.5s-3.7 26-16.5 29.7l-206.6 59 0 109.9 136 0c35.3 0 64 28.7 64 64l0 160c0 35.3-28.7 64-64 64L96 512c-35.3 0-64-28.7-64-64l0-160zm80 0c-8.8 0-16 7.2-16 16l0 64c0 8.8 7.2 16 16 16l56 0 0-96-56 0zm184 96l0-96-80 0 0 96 80 0zm48 0l56 0c8.8 0 16-7.2 16-16l0-64c0-8.8-7.2-16-16-16l-56 0 0 96z",
            }
        }
    }
}
