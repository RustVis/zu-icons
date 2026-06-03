// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SpeakapProps {
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

pub fn Speakap(props: SpeakapProps) -> Element {
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
        d: "M64.4 391.8C-15 303.6-7.6 167.4 81.1 87.6s224.8-73 304.2 15.2 72 224.4-16.6 304.1c-18.7 16.9 64 43.1 42 52.3-82.1 34.2-253.9 35-346.2-67.5l0 0zM277.7 180.2l38.5-40.9c-9.6-8.9-32-26.8-76.2-27.6-52.3-.9-95.9 28.3-96.8 80-.2 11.3 .3 36.7 29.4 54.8 34.5 21.4 86.5 21.5 86 52.3-.4 21.3-26.4 25.8-38.6 25.6-3 0-30.2-.5-47.6-24.6l-40 42.6c28.2 27 59 32.6 83.5 33 10.2 .2 96.4 .3 97.8-81 .3-15.8-2.1-39.7-28.9-56.6-34.4-21.6-85-19.4-84.4-49.7 .4-23.3 31-25.4 37.5-25.3 .4 0 26.6 .3 39.6 17.4l0 0z",
            }
        }
    }
}
