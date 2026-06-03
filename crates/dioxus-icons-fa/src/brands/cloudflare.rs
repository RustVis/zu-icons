// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CloudflareProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Cloudflare(props: CloudflareProps) -> Element {
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
        d: "M407.9 319.9L177.1 317c-.7 0-1.4-.2-2-.5s-1.2-.8-1.6-1.4c-.4-.6-.7-1.3-.7-2s0-1.4 .2-2.1c.4-1.1 1.1-2.1 2.1-2.8s2.1-1.2 3.3-1.2l232.9-2.9c27.6-1.3 57.5-23.6 68-50.8l13.3-34.5c.4-.9 .5-1.9 .5-2.9 0-.5-.1-1.1-.2-1.6-7.4-32.2-25-61.1-50.3-82.3s-56.7-33.7-89.7-35.5-65.6 7.3-93 25.7-48 45.3-58.8 76.5c-11.3-8.5-24.9-13.3-39-13.7s-28 3.5-39.8 11.4-20.8 19.1-25.9 32.3-5.9 27.6-2.4 41.3c-52.3 1.5-94.2 44.1-94.2 96.5 0 4.7 .3 9.3 1 14 .2 1.1 .7 2.1 1.5 2.8s1.9 1.1 2.9 1.1l426.1 .1c0 0 .1 0 .1 0 1.2 0 2.3-.4 3.3-1.1s1.6-1.7 2-2.9l3.3-11.3c3.9-13.4 2.4-25.8-4.1-34.9-6-8.4-16.1-13.3-28.2-13.9zm105.9-98.8c-2.1 0-4.3 .1-6.4 .2-.8 .1-1.5 .3-2.1 .8s-1 1.1-1.3 1.8l-9.1 31.2c-3.9 13.4-2.4 25.8 4.1 34.9 6 8.4 16.1 13.3 28.2 13.9l49.2 2.9c.7 0 1.4 .2 2 .5s1.1 .8 1.5 1.4c.4 .6 .7 1.3 .8 2s0 1.5-.2 2.1c-.4 1.1-1.1 2.1-2.1 2.8s-2.1 1.2-3.3 1.2l-51.1 2.9c-27.8 1.3-57.7 23.6-68.1 50.8l-3.7 9.6c-.2 .4-.2 .8-.2 1.3s.2 .8 .4 1.2 .6 .7 .9 .9 .8 .3 1.2 .3c0 0 .1 0 .1 0l175.9 0c1 0 2-.3 2.8-.9s1.4-1.5 1.7-2.4c3.1-11.1 4.7-22.5 4.7-34 0-69.3-56.5-125.5-126.1-125.5z",
            }
        }
    }
}
