// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HamsaProps {
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

pub fn Hamsa(props: HamsaProps) -> Element {
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
        d: "M34.6 288L80 288c8.8 0 16-7.2 16-16L96 72c0-22.1 17.9-40 40-40s40 17.9 40 40l0 132c0 11 9 20 20 20s20-9 20-20l0-164c0-22.1 17.9-40 40-40s40 17.9 40 40l0 164c0 11 9 20 20 20s20-9 20-20l0-132c0-22.1 17.9-40 40-40s40 17.9 40 40l0 200c0 8.8 7.2 16 16 16l45.4 0c19.1 0 34.6 15.5 34.6 34.6 0 8.6-3.2 16.9-9 23.3L416.6 441c-41.1 45.2-99.4 71-160.6 71S136.6 486.2 95.4 441L9 345.9c-5.8-6.4-9-14.7-9-23.3 0-19.1 15.5-34.6 34.6-34.6zM256 288c-38.4 0-76.8 35.8-90.6 50.2-3.6 3.7-5.4 8.7-5.4 13.8s1.8 10.1 5.4 13.8C179.2 380.2 217.6 416 256 416s76.8-35.8 90.6-50.2c3.6-3.7 5.4-8.7 5.4-13.8s-1.8-10.1-5.4-13.8C332.8 323.8 294.4 288 256 288zm0 32a32 32 0 1 1 0 64 32 32 0 1 1 0-64z",
            }
        }
    }
}
