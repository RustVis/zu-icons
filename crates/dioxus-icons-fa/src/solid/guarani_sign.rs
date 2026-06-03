// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GuaraniSignProps {
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

pub fn GuaraniSign(props: GuaraniSignProps) -> Element {
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
        d: "M192 0c-13.3 0-24 10.7-24 24l0 41.5C73.3 77.3 0 158.1 0 256S73.3 434.7 168 446.5l0 41.5c0 13.3 10.7 24 24 24s24-10.7 24-24l0-41.5c94.7-11.8 168-92.6 168-190.5 0-17.7-14.3-32-32-32l-136 0 0-93.8c25.3 4.8 47.9 17 65.6 34.3 12.6 12.4 32.9 12.2 45.3-.4s12.2-32.9-.5-45.3C297.2 90.2 258.8 70.8 216 65.5L216 24c0-13.3-10.7-24-24-24zM168 130.2l0 251.5C108.8 370.5 64 318.5 64 256s44.8-114.5 104-125.8zm48 251.5l0-93.8 100 0c-12.3 47.6-51.2 84.5-100 93.8z",
            }
        }
    }
}
