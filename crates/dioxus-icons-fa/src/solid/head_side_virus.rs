// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HeadSideVirusProps {
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

pub fn HeadSideVirus(props: HeadSideVirusProps) -> Element {
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
        d: "M329.7 448c-5.1 0-9.7 3.7-9.7 8.8l0 7.2c0 26.5-21.5 48-48 48l-160 0c-26.5 0-48-21.5-48-48l0-63.6c0-12.6-5.1-24.5-13.1-34.2-31.8-38.7-50.9-88.2-50.9-142.2 0-123.7 100.3-224 224-224 112.7 0 206 83.3 221.7 191.7 .4 3 1.7 5.9 3.6 8.3l35.8 42.9c7 8.4 10.9 19.1 10.9 30.1 0 25.9-21 47-47 47l-1 0c-8.8 0-16 7.2-16 16l0 48c0 35.3-28.7 64-64 64l-38.3 0zM224 64c-13.3 0-24 10.7-24 24 0 22.9-27.7 34.4-43.9 18.2-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9c16.2 16.2 4.7 43.9-18.2 43.9-13.3 0-24 10.7-24 24s10.7 24 24 24c22.9 0 34.4 27.7 18.2 43.9-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0c16.2-16.2 43.9-4.7 43.9 18.2 0 13.3 10.7 24 24 24s24-10.7 24-24c0-22.9 27.7-34.4 43.9-18.2 9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9c-16.2-16.2-4.7-43.9 18.2-43.9 13.3 0 24-10.7 24-24s-10.7-24-24-24c-22.9 0-34.4-27.7-18.2-43.9 9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0c-16.2 16.2-43.9 4.7-43.9-18.2 0-13.3-10.7-24-24-24zm-32 88a24 24 0 1 1 0 48 24 24 0 1 1 0-48zm40 88a24 24 0 1 1 48 0 24 24 0 1 1 -48 0z",
            }
        }
    }
}
