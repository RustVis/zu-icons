// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct VialVirusProps {
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

    #[props(default = Some("0 0 576 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn VialVirus(props: VialVirusProps) -> Element {
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
        d: "M64 32C64 14.3 78.3 0 96 0L320 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l0 151.7c-18.5 0-37 7-51.1 21.1-21.6 21.6-26.6 53.6-15 79.9-26.9 10.4-45.9 36.6-45.9 67.1s19 56.7 45.9 67.1c-7.3 16.5-8 35.2-2.3 52.2-13.4 5.7-28.2 8.8-43.6 8.8-61.9 0-112-50.1-112-112L96 64C78.3 64 64 49.7 64 32zm96 32l0 128 96 0 0-128-96 0zM280 408c-13.3 0-24-10.7-24-24s10.7-24 24-24c28.8 0 43.2-34.8 22.9-55.2-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0c20.4 20.4 55.2 5.9 55.2-22.9 0-13.3 10.7-24 24-24s24 10.7 24 24c0 28.8 34.8 43.2 55.2 22.9 9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9c-20.4 20.4-5.9 55.2 22.9 55.2 13.3 0 24 10.7 24 24s-10.7 24-24 24c-28.8 0-43.2 34.8-22.9 55.2 9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0c-20.4-20.4-55.2-5.9-55.2 22.9 0 13.3-10.7 24-24 24s-24-10.7-24-24c0-28.8-34.8-43.2-55.2-22.9-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9c20.4-20.4 5.9-55.2-22.9-55.2zm104-32a24 24 0 1 0 0-48 24 24 0 1 0 0 48zm88 40a24 24 0 1 0 -48 0 24 24 0 1 0 48 0z",
            }
        }
    }
}
