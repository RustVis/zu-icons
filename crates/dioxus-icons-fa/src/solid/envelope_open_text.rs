// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct EnvelopeOpenTextProps {
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

pub fn EnvelopeOpenText(props: EnvelopeOpenTextProps) -> Element {
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
        d: "M288 33.9L96.4 175.8 254.5 293c5.3 3.9 11.2 6.9 17.5 8.7L272 464c0 5.5 .5 10.8 1.3 16L96 480c-35.3 0-64-28.7-64-64l0-239.9c0-20.3 9.6-39.4 25.9-51.4L254.5-21c9.7-7.2 21.4-11 33.5-11s23.8 3.9 33.5 11L518.1 124.7c7.2 5.3 13.1 12 17.4 19.6-2.5-.2-5-.3-7.5-.3L436.6 144 288 33.9zM320 240c0-26.5 21.5-48 48-48l160 0c26.5 0 48 21.5 48 48l0 224c0 26.5-21.5 48-48 48l-160 0c-26.5 0-48-21.5-48-48l0-224zm80 16c-13.3 0-24 10.7-24 24s10.7 24 24 24l96 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-96 0zm0 96c-13.3 0-24 10.7-24 24s10.7 24 24 24l56 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-56 0z",
            }
        }
    }
}
