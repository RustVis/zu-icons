// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UmbracoProps {
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

pub fn Umbraco(props: UmbracoProps) -> Element {
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
        d: "M256.3 8a248 248 0 1 0 -.7 496 248 248 0 1 0 .7-496zm145 266c-.8 27.1-5.4 48.8-14 65s-23.1 27.9-43.5 35c-20.4 7.1-48.9 10.6-85.4 10.5l-4.6 0c-36.5 .1-65-3.3-85.4-10.5s-34.9-18.8-43.5-35c-8.6-16.2-13.2-37.9-14-65-.7-10.2-.7-20.5 0-30.7 .4-14.7 1.6-29.3 3.6-43.9 1.9-13.4 3.6-22.6 5.4-32 1-4.9 1.3-6.4 1.8-8.4 .3-1.1 .9-2.1 1.8-2.8s2-1.1 3.1-1.1l.7 0 32 5c1.2 .2 2.2 .8 3 1.7s1.2 2.1 1.2 3.3c0 .3 0 .5 0 .8l-1.7 8.8c-1.6 8.8-3.2 20.1-4.8 33.7-1.7 14-2.5 28.1-2.6 42.2-.2 27 2.5 46.9 8.1 59.8 2.8 6.4 7.2 12 12.6 16.4s11.9 7.4 18.7 8.8c18.8 4 38.1 5.7 57.3 5.1l10.3 0c19.2 .6 38.5-1.2 57.3-5.2 6.8-1.4 13.2-4.5 18.6-8.8s9.7-10 12.5-16.4c5.7-12.9 8.4-32.9 8.1-59.8-.1-14.1-1-28.2-2.6-42.1-1.7-13.6-3.3-24.8-4.9-33.7l-1.7-8.8c0-.3 0-.5 0-.8 0-1.2 .4-2.4 1.2-3.3s1.8-1.5 3-1.7l32-5 .8 0c1.1 0 2.2 .4 3.1 1.1s1.5 1.7 1.8 2.8c.6 2 .8 3.6 1.8 8.4 1.8 9.6 3.5 18.8 5.4 32 2 14.6 3.2 29.2 3.6 43.9 .7 10.2 .7 20.5 0 30.7l0 0z",
            }
        }
    }
}
