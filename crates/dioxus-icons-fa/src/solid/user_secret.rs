// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UserSecretProps {
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

pub fn UserSecret(props: UserSecretProps) -> Element {
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
        d: "M171-16c-36.4 0-57.8 58.3-68.3 112L72 96c-13.3 0-24 10.7-24 24s10.7 24 24 24l24 0 0 32c0 17 3.3 33.2 9.3 48l-9.3 0 0 0-20.5 0c-15.2 0-27.5 12.3-27.5 27.5 0 3 .5 5.9 1.4 8.7l28.9 86.6C40.2 379.6 16 428.1 16 482.3 16 498.7 29.3 512 45.7 512l356.6 0c16.4 0 29.7-13.3 29.7-29.7 0-54.2-24.2-102.7-62.3-135.4l28.9-86.6c.9-2.8 1.4-5.7 1.4-8.7 0-15.2-12.3-27.5-27.5-27.5l-20.5 0 0 0-9.3 0c6-14.8 9.3-31 9.3-48l0-32 24 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-30.7 0c-10.4-53.7-31.9-112-68.3-112-9.6 0-19 3.9-27.5 8.2-8.2 4.1-18.4 7.8-25.5 7.8s-17.3-3.7-25.5-7.8C190-12.1 180.6-16 171-16zm93.7 484.4l-24.8-70.9 27.9-32.5c2.7-3.2 4.2-7.2 4.2-11.4 0-9.7-7.8-17.5-17.5-17.5l-61 0c-9.7 0-17.5 7.8-17.5 17.5 0 4.2 1.5 8.2 4.2 11.4l27.9 32.5-24.8 70.9-57-180.4 35.7 0c18.4 10.2 39.5 16 62 16s43.6-5.8 62-16l35.7 0-57 180.4zM224 256c-34.7 0-64.2-22.1-75.3-53 5.7 3.2 12.3 5 19.3 5l12.4 0c16.5 0 31.1-10.6 36.3-26.2 2.3-7 12.2-7 14.5 0 5.2 15.6 19.9 26.2 36.3 26.2l12.4 0c7 0 13.6-1.8 19.3-5-11.1 30.9-40.6 53-75.3 53z",
            }
        }
    }
}
