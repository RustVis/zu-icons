// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AdProps {
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

pub fn Ad(props: AdProps) -> Element {
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
        d: "M64 64C28.7 64 0 92.7 0 128L0 384c0 35.3 28.7 64 64 64l384 0c35.3 0 64-28.7 64-64l0-256c0-35.3-28.7-64-64-64L64 64zM416 184l0 144c0 13.3-10.7 24-24 24-7.1 0-13.5-3.1-17.9-8-10.2 5.1-21.8 8-34.1 8-42 0-76-34-76-76s34-76 76-76c9.9 0 19.3 1.9 28 5.3l0-21.3c0-13.3 10.7-24 24-24s24 10.7 24 24zm-48 92a28 28 0 1 0 -56 0 28 28 0 1 0 56 0zM160 208c-8.8 0-16 7.2-16 16l0 32 48 0 0-32c0-8.8-7.2-16-16-16l-16 0zm32 96l-48 0 0 24c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-104c0-35.3 28.7-64 64-64l16 0c35.3 0 64 28.7 64 64l0 104c0 13.3-10.7 24-24 24s-24-10.7-24-24l0-24z",
            }
        }
    }
}
