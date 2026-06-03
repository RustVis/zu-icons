// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DiagnosesProps {
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

pub fn Diagnoses(props: DiagnosesProps) -> Element {
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
        d: "M184 72a72 72 0 1 1 144 0 72 72 0 1 1 -144 0zM160 299.3c-19.9 22.6-32 52.2-32 84.7l0 32-64 0 0-32c0-106 86-192 192-192s192 86 192 192l0 32-64 0 0-32c0-32.5-12.1-62.1-32-84.7l0 116.7-192 0 0-116.7zM232 384a24 24 0 1 0 0-48 24 24 0 1 0 0 48zm88-104a24 24 0 1 0 -48 0 24 24 0 1 0 48 0zM24 464l464 0c13.3 0 24 10.7 24 24s-10.7 24-24 24L24 512c-13.3 0-24-10.7-24-24s10.7-24 24-24zM64 184a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zm424 72a24 24 0 1 1 0 48 24 24 0 1 1 0-48z",
            }
        }
    }
}
