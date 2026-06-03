// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BahtSignProps {
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

    #[props(default = Some("0 0 320 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn BahtSign(props: BahtSignProps) -> Element {
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
        d: "M136 0c-13.3 0-24 10.7-24 24l0 40-74.4 0C16.8 64 0 80.8 0 101.6L0 406.3c0 23 18.7 41.7 41.7 41.7l70.3 0 0 40c0 13.3 10.7 24 24 24s24-10.7 24-24l0-40 48 0c61.9 0 112-50.1 112-112 0-40.1-21.1-75.3-52.7-95.1 13.1-18.3 20.7-40.7 20.7-64.9 0-61.9-50.1-112-112-112l-16 0 0-40c0-13.3-10.7-24-24-24zM112 128l0 96-48 0 0-96 48 0zm48 96l0-96 16 0c26.5 0 48 21.5 48 48s-21.5 48-48 48l-16 0zm-48 64l0 96-48 0 0-96 48 0zm48 96l0-96 48 0c26.5 0 48 21.5 48 48s-21.5 48-48 48l-48 0z",
            }
        }
    }
}
