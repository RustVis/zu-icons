// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SymfonyProps {
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

pub fn Symfony(props: SymfonyProps) -> Element {
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
        d: "M256 8a248 248 0 1 0 0 496 248 248 0 1 0 0-496zM389.7 151.5c-11.5 .4-19.4-6.4-19.8-16.9-.3-9.2 6.7-13.4 6.5-18.9-.2-6.5-10.2-6.8-12.9-6.7-39.8 1.3-48.6 57-58.9 113.8 21.4 3.2 36.6-.7 45.1-6.2 12-7.7-3.3-15.7-1.4-24.6 4-18.2 32.6-19 32 5.3-.4 17.9-25.9 41.8-77.6 35.7-10.8 59.5-18.4 115-58.2 161.7-29 34.5-58.4 39.8-71.6 40.3-24.6 .9-41-12.3-41.6-29.8-.6-17 14.4-26.3 24.3-26.6 21.9-.8 30.1 25.7 14.9 34-12.1 9.7 .1 12.6 2.1 12.6 10.4-.4 17.3-5.5 22.2-9 24-20 33.2-54.9 45.4-118.3 8.2-49.7 17-78 18.2-82-16.9-12.7-27.1-28.6-49.8-34.7-15.6-4.2-25.1-.6-31.8 7.8-7.9 10-5.3 23 2.4 30.7l12.6 14c15.5 17.9 24 31.9 20.8 50.6-5.1 29.9-40.7 52.9-82.9 39.9-36-11.1-42.7-36.6-38.4-50.6 7.5-24.2 42.4-11.7 34.6 13.6-2.8 8.6-4.9 8.7-6.3 13.1-4.6 14.8 41.8 28.4 51-1.4 4.5-14.5-5.3-21.7-22.2-39.9-28.5-31.7-16-65.5 2.9-79.7 52.8-39.4 100.5 17.5 110.6 25.8 37.2-109 100.5-105.5 102.4-105.5 25.2-.8 44.2 10.6 44.8 28.6 .2 7.7-4.2 22.6-19.5 23.1z",
            }
        }
    }
}
