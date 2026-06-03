// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AudioDescriptionProps {
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

pub fn AudioDescription(props: AudioDescriptionProps) -> Element {
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
        d: "M0 128C0 92.7 28.7 64 64 64l384 0c35.3 0 64 28.7 64 64l0 256c0 35.3-28.7 64-64 64L64 448c-35.3 0-64-28.7-64-64L0 128zm96 96l0 104c0 13.3 10.7 24 24 24s24-10.7 24-24l0-24 48 0 0 24c0 13.3 10.7 24 24 24s24-10.7 24-24l0-104c0-35.3-28.7-64-64-64l-16 0c-35.3 0-64 28.7-64 64zm48 32l0-32c0-8.8 7.2-16 16-16l16 0c8.8 0 16 7.2 16 16l0 32-48 0zm152-96c-13.3 0-24 10.7-24 24l0 144c0 13.3 10.7 24 24 24l48 0c39.8 0 72-32.2 72-72l0-48c0-39.8-32.2-72-72-72l-48 0zm48 144l-24 0 0-96 24 0c13.3 0 24 10.7 24 24l0 48c0 13.3-10.7 24-24 24z",
            }
        }
    }
}
