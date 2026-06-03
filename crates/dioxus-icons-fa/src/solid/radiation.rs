// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RadiationProps {
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

pub fn Radiation(props: RadiationProps) -> Element {
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
        d: "M446.2 34.5c-14.2-10.1-33.5-4.6-42.2 10.5L331.6 170.3c31.3 15.8 52.8 48.3 52.8 85.7l144 0c17.7 0 32.2-14.4 30.1-31.9-9.1-78.1-51.4-146.1-112.3-189.6zM172.7 44.9C164 29.8 144.7 24.3 130.5 34.5 69.6 77.9 27.3 145.9 18.2 224.1 16.1 241.6 30.7 256 48.3 256l144 0c0-37.5 21.5-69.9 52.8-85.7L172.7 44.9zm-9.4 416.8c-8.7 15.1-3.8 34.5 12 41.8 34.4 15.7 72.7 24.5 113 24.5s78.6-8.8 113-24.5c15.8-7.2 20.7-26.7 12-41.8L341 336.3c-15.1 9.9-33.2 15.7-52.6 15.7s-37.5-5.8-52.6-15.7L163.3 461.7zM288.3 304a48 48 0 1 0 -.7-96 48 48 0 1 0 .7 96z",
            }
        }
    }
}
