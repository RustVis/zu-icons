// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PassportProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Passport(props: PassportProps) -> Element {
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
        d: "M0 64C0 28.7 28.7 0 64 0L320 0c35.3 0 64 28.7 64 64l0 384c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 64zM96 408c0 13.3 10.7 24 24 24l144 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-144 0c-13.3 0-24 10.7-24 24zM278.6 208c-4.8 26.4-21.5 48.7-44.2 61.2 6.7-17 11.2-38 12.6-61.2l31.6 0zm-173.1 0l31.6 0c1.4 23.1 6 44.2 12.6 61.2-22.7-12.5-39.4-34.8-44.2-61.2zm76.4 55c-6.2-13.4-11.1-32.5-12.7-55l45.8 0c-1.6 22.5-6.5 41.6-12.7 55-4.5 9.6-8.2 13.8-10.2 15.5-2-1.7-5.7-5.8-10.2-15.5zm0-142c4.5-9.6 8.2-13.8 10.2-15.5 2 1.7 5.7 5.8 10.2 15.5 6.2 13.4 11.1 32.5 12.7 55l-45.8 0c1.6-22.5 6.5-41.6 12.7-55zm96.7 55L247 176c-1.4-23.1-6-44.2-12.6-61.2 22.7 12.5 39.4 34.8 44.2 61.2zM137 176l-31.6 0c4.8-26.4 21.5-48.7 44.2-61.2-6.7 17-11.2 38-12.6 61.2zm183 16a128 128 0 1 0 -256 0 128 128 0 1 0 256 0z",
            }
        }
    }
}
