// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct VirusCovidSlashProps {
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

pub fn VirusCovidSlash(props: VirusCovidSlashProps) -> Element {
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
        d: "M41-24.9c-9.4-9.4-24.6-9.4-33.9 0S-2.3-.3 7 9.1l528 528c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-83-83 11.4-11.4c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-11.3 11.3-23.8-23.8c17.9-23.5 29.9-51.7 34.1-82.3l33.6 0 0 16c0 13.3 10.7 24 24 24s24-10.7 24-24l0-80c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 16-33.6 0c-4.2-30.7-16.3-58.8-34.1-82.3l23.8-23.8 11.3 11.3c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9L440.7 46.7c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9l11.3 11.3-23.8 23.8C370.8 97.9 342.7 85.8 312 81.6l0-33.6 16 0c13.3 0 24-10.7 24-24S341.3 0 328 0L248 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l16 0 0 33.6c-30.7 4.2-58.8 16.3-82.3 34.1L157.9 92 169.2 80.6c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0L123.9 58.1 41-24.9zM113.6 232l-33.6 0 0-16c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 80c0 13.3 10.7 24 24 24s24-10.7 24-24l0-16 33.6 0c4.2 30.7 16.3 58.8 34.1 82.3l-23.8 23.8-11.3-11.3c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9l56.6 56.6c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-11.3-11.3 23.8-23.8c23.5 17.9 51.7 29.9 82.3 34.1l0 33.6-16 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l80 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-16 0 0-33.6c13.4-1.8 26.4-5.2 38.7-9.9L123.5 193.3c-4.7 12.3-8 25.2-9.9 38.7z",
            }
        }
    }
}
