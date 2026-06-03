// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ShopwareProps {
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

pub fn Shopware(props: ShopwareProps) -> Element {
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
        d: "M403.5 455.4c-42.6 31.7-94.4 48.7-147.5 48.6-137.2 0-248-111-248-248 0-137.2 111-248 248-248 61.2-.1 120.2 22.6 165.7 63.5 .6 .5 .9 1.2 1.1 1.9s.1 1.5-.3 2.2-.8 1.3-1.5 1.6-1.4 .5-2.2 .4c-18.8-2.5-37.7-3.7-56.7-3.7-129.4 0-222.4 53.5-222.4 155.4 0 109 92.1 145.9 176.8 178.7 33.6 13 65.4 25.4 87 41.6 .4 .3 .8 .8 1.1 1.3s.4 1 .4 1.6-.1 1.1-.4 1.6-.6 .9-1.1 1.3l-.1 0zM503 233.1c-.1-.9-.5-1.8-1.3-2.4-51.8-43-93.6-60.5-144.5-60.5-84.1 0-80.3 52.2-80.3 53.6 0 42.6 52.1 62 112.3 84.5 31.1 11.6 63.2 23.6 92.7 39.9 .4 .2 .9 .4 1.4 .4s1 0 1.5-.2 .9-.4 1.3-.8 .6-.8 .8-1.2c14.1-36 19.5-74.8 16-113.3z",
            }
        }
    }
}
