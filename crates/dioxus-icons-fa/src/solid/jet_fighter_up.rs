// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct JetFighterUpProps {
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

pub fn JetFighterUp(props: JetFighterUpProps) -> Element {
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
        d: "M206.8 47.8C202.3 58.5 200 70 200 81.6l0 100.4-152 114 0-48c0-13.3-10.7-24-24-24S0 234.7 0 248L0 392c0 13.3 10.7 24 24 24s24-10.7 24-24l0-8 152 0 0 54.4-66 52.8c-3.8 3-6 7.6-6 12.5l0 24.3c0 8.8 7.2 16 16 16l88 0 0-40c0-13.3 10.7-24 24-24s24 10.7 24 24l0 40 88 0c8.8 0 16-7.2 16-16l0-24.3c0-4.9-2.2-9.5-6-12.5l-66-52.8 0-54.4 152 0 0 8c0 13.3 10.7 24 24 24s24-10.7 24-24l0-144c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 48-152-114 0-100.4c0-11.6-2.3-23.1-6.8-33.8l-27.1-65C274.4-26.2 265.7-32 256-32s-18.4 5.8-22.2 14.8l-27.1 65z",
            }
        }
    }
}
