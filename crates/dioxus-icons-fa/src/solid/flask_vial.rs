// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FlaskVialProps {
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

pub fn FlaskVial(props: FlaskVialProps) -> Element {
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
        d: "M184.6 411.5c-3.1 7.3-5.4 14.9-6.8 22.6-14.5 8.8-31.5 13.9-49.8 13.9-53 0-96-43-96-96L32 64C14.3 64 0 49.7 0 32S14.3 0 32 0L224 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l0 255.6-39.4 91.9zM96 64l0 128 64 0 0-128-64 0zM352 0L512 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l0 153.4 91.3 213c2.3 5.4 3.8 11.1 4.4 17l.3 .6-.3 0c.2 1.8 .3 3.6 .3 5.4 0 32.3-26.2 58.6-58.6 58.6l-266.9 0c-32.3 0-58.6-26.2-58.6-58.6 0-1.8 .1-3.6 .3-5.4l-.3 0 .3-.6c.6-5.8 2.1-11.6 4.4-17L320 217.4 320 64c-17.7 0-32-14.3-32-32S302.3 0 320 0l32 0zM453.2 242.6c-3.4-8-5.2-16.5-5.2-25.2l0-153.4-64 0 0 153.4c0 8.7-1.8 17.2-5.2 25.2l-33.2 77.4 140.7 0-33.2-77.4z",
            }
        }
    }
}
