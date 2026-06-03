// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MintbitProps {
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

pub fn Mintbit(props: MintbitProps) -> Element {
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
        d: "M73.2 512l0-73.1 292.5 0 0-73.2 73.2 0 0-146.3 73.1 0 0-219.4-219.4 0 0 73.1-146.3 0 0 73.2-73.1 0 0 292.6-73.2 0 0 73.1 73.2 0zm73.1-219.4l73.2 0 0 73.1-73.2 0 0-73.1zm73.2-73.1l73.1 0 0 73.1-73.2 0 0-73.2 .1 .1zm73.1 0l0-73.2 73.2 0 0 73.1-73.2 0 0 .1zM365.7 73.1l73.2 0 0 73.2-73.2 0 0-73.2z",
            }
        }
    }
}
