// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MixerProps {
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

pub fn Mixer(props: MixerProps) -> Element {
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
        d: "M82.6 76.1c-3.8-5.1-8.7-9.4-14.2-12.5s-11.7-5.1-18.1-5.7-12.8 .1-18.8 2.2-11.6 5.3-16.3 9.6c-17.6 16.2-19 43.5-4.8 62.8l91.8 123-92.3 124.1c-14.2 19.3-13.1 46.6 4.7 62.8 4.7 4.3 10.3 7.6 16.3 9.6s12.5 2.8 18.8 2.2 12.5-2.5 18.1-5.7 10.4-7.4 14.2-12.5L210.9 262.7c1.5-2.1 2.3-4.6 2.3-7.1s-.8-5-2.3-7.1L82.6 76.1zM438.2 379.6l-92.3-124.1 91.8-123c14.2-19.2 12.8-46.6-4.7-62.8-4.7-4.3-10.3-7.6-16.3-9.6s-12.5-2.8-18.8-2.2-12.5 2.5-18.1 5.7-10.4 7.4-14.2 12.5l-128 172.1c-1.5 2.1-2.3 4.6-2.3 7.1s.8 5 2.3 7.1L366 435.9c3.8 5.1 8.7 9.4 14.2 12.5s11.7 5.1 18.1 5.7 12.8-.1 18.8-2.2 11.6-5.3 16.3-9.6c17.8-16.2 19-43.5 4.7-62.8z",
            }
        }
    }
}
