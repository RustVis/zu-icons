// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SuitcaseMedicalProps {
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

pub fn SuitcaseMedical(props: SuitcaseMedicalProps) -> Element {
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
        d: "M192 56l0 40 128 0 0-40c0-4.4-3.6-8-8-8L200 48c-4.4 0-8 3.6-8 8zm-48 8l0-8c0-30.9 25.1-56 56-56L312 0c30.9 0 56 25.1 56 56l0 424-224 0 0-416zM96 176l0 304-32 0c-35.3 0-64-28.7-64-64L0 160c0-35.3 28.7-64 64-64l32 0 0 80zM416 480l0-384 32 0c35.3 0 64 28.7 64 64l0 256c0 35.3-28.7 64-64 64l-32 0zM244 208c-8.8 0-16 7.2-16 16l0 36-36 0c-8.8 0-16 7.2-16 16l0 24c0 8.8 7.2 16 16 16l36 0 0 36c0 8.8 7.2 16 16 16l24 0c8.8 0 16-7.2 16-16l0-36 36 0c8.8 0 16-7.2 16-16l0-24c0-8.8-7.2-16-16-16l-36 0 0-36c0-8.8-7.2-16-16-16l-24 0z",
            }
        }
    }
}
