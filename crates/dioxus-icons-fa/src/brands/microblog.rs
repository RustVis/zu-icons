// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MicroblogProps {
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

pub fn Microblog(props: MicroblogProps) -> Element {
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
        d: "M399.8 362.2c29.5-34.7 47.1-78.3 47.1-125.8 0-113-99.6-204.4-222.5-204.4S2 123.5 2 236.4 101.6 440.9 224.5 440.9c27 0 53.9-4.5 79.4-13.4 1.4-.5 3-.5 4.5-.1s2.7 1.4 3.6 2.6c18.6 25.1 47.6 42.7 79.9 49.9 1.1 .2 2.3 0 3.3-.6s1.7-1.6 1.9-2.8c.1-.6 .1-1.3 0-1.9s-.4-1.2-.8-1.7c-12.3-16-18.7-35.8-18-56s8.4-39.5 21.7-54.7l-.2 .1zM330 212.4l-57.3 43.5 20.8 68.9c.4 1.3 .4 2.7-.1 4s-1.2 2.4-2.3 3.2-2.4 1.2-3.8 1.2-2.7-.4-3.8-1.2l-59.1-41-59.1 41.1c-1.1 .8-2.4 1.2-3.8 1.2s-2.7-.4-3.8-1.2-1.9-1.9-2.3-3.2-.5-2.7-.1-4l20.8-68.9-57.3-43.5c-1.1-.8-1.9-1.9-2.3-3.2s-.4-2.7 0-4 1.2-2.4 2.3-3.2 2.4-1.3 3.7-1.3l71.9-1.5 23.7-67.9c.4-1.3 1.3-2.4 2.4-3.2s2.4-1.2 3.8-1.2 2.7 .4 3.8 1.2 1.9 1.9 2.4 3.2l23.7 67.9 71.9 1.5c1.4 0 2.7 .4 3.8 1.2s1.9 1.9 2.3 3.2 .4 2.7 0 4-1.2 2.4-2.3 3.3l0 0z",
            }
        }
    }
}
