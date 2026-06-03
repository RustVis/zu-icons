// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PesoSignProps {
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

    #[props(default = Some("0 0 448 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn PesoSign(props: PesoSignProps) -> Element {
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
        d: "M112 32C94.3 32 80 46.3 80 64l0 64-24 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l24 0 0 32-24 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l24 0 0 192c0 17.7 14.3 32 32 32s32-14.3 32-32l0-96 96 0c65.6 0 122-39.5 146.7-96l37.3 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-24.8 0c.5-5.3 .8-10.6 .8-16s-.3-10.7-.8-16l24.8 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-37.3 0C362 71.5 305.6 32 240 32L112 32zm199.6 96l-167.6 0 0-32 96 0c28.4 0 54 12.4 71.6 32zM144 176l190.7 0c.9 5.2 1.3 10.5 1.3 16s-.5 10.8-1.3 16l-190.7 0 0-32zm167.6 80c-17.6 19.6-43.1 32-71.6 32l-96 0 0-32 167.6 0z",
            }
        }
    }
}
