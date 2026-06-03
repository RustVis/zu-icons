// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PlaneSlashProps {
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

pub fn PlaneSlash(props: PlaneSlashProps) -> Element {
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
        d: "M41-24.9c-9.4-9.4-24.6-9.4-33.9 0S-2.3-.3 7 9.1l528 528c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-183.4-183.4 7.1-7.7 127.3 0c30.9 0 56-25.1 56-56s-25.1-56-56-56L392.7 200 233.5 26.4C227.5 19.8 218.9 16 209.9 16l-43.7 0c-10.9 0-18.6 10.7-15.2 21.1L175.1 109.2 41-24.9zM130.2 200l-24.6 0-52.8-66c-3-3.8-7.6-6-12.5-6l-19.8 0c-10.4 0-18 9.8-15.5 19.9L32 256 5 364.1C2.4 374.2 10.1 384 20.5 384l19.8 0c4.9 0 9.5-2.2 12.5-6l52.8-66 99.7 0-54.3 162.9c-3.5 10.4 4.3 21.1 15.2 21.1l43.7 0c9 0 17.5-3.8 23.6-10.4L320.7 390.5 130.2 200z",
            }
        }
    }
}
