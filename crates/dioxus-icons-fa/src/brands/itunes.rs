// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ItunesProps {
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

pub fn Itunes(props: ItunesProps) -> Element {
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
        d: "M223.6 80.3a171.3 171.3 0 1 0 .2 342.5 171.3 171.3 0 1 0 -.2-342.5zm79.4 240c-3.2 13.6-13.5 21.2-27.3 23.8-12.1 2.2-22.2 2.8-31.9-5-11.8-10-12-26.4-1.4-36.8 8.4-8 20.3-9.6 38-12.8 3-.5 5.6-1.2 7.7-3.7 3.2-3.6 2.2-2 2.2-80.8 0-5.6-2.7-7.1-8.4-6.1-4 .7-91.9 17.1-91.9 17.1-5 1.1-6.7 2.6-6.7 8.3 0 116.1 .5 110.8-1.2 118.5-2.1 9-7.6 15.8-14.9 19.6-8.3 4.6-23.4 6.6-31.4 5.2-21.4-4-28.9-28.7-14.4-42.9 8.4-8 20.3-9.6 38-12.8 3-.5 5.6-1.2 7.7-3.7 5-5.7 .9-127 2.6-133.7 .4-2.6 1.5-4.8 3.5-6.4 2.1-1.7 5.8-2.7 6.7-2.7 101-19 113.3-21.4 115.1-21.4 5.7-.4 9 3 9 8.7-.1 170.6 .4 161.4-1 167.6zM345.2 32L102.8 32C45.9 32 0 77.9 0 134.8L0 377.2C0 434.1 45.9 480 102.8 480l242.4 0c57 0 102.8-45.9 102.8-102.8l0-242.4C448 77.9 402.1 32 345.2 32zM223.6 59a192.5 192.5 0 1 1 0 385 192.5 192.5 0 1 1 0-385z",
            }
        }
    }
}
