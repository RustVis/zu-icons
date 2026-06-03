// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DevProps {
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

pub fn Dev(props: DevProps) -> Element {
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
        d: "M120.1 208.3c-3.9-2.9-7.8-4.3-11.6-4.3l-17.4 0 0 104.5 17.4 0c3.9 0 7.8-1.4 11.6-4.3s5.8-7.3 5.8-13.1l0-69.7c0-5.8-2-10.2-5.8-13.1zM404.1 32L43.9 32C19.7 32 .1 51.6 0 75.8L0 436.2C.1 460.4 19.7 480 43.9 480l360.2 0c24.2 0 43.8-19.6 43.9-43.8l0-360.4C447.9 51.6 428.3 32 404.1 32zM154.2 291.2c0 18.8-11.6 47.3-48.4 47.3l-46.4 0 0-165.5 47.4 0c35.4 0 47.4 28.5 47.4 47.3l0 70.9zm100.7-88.7l-53.3 0 0 38.4 32.6 0 0 29.6-32.6 0 0 38.4 53.3 0 0 29.6-62.2 0c-11.2 .3-20.4-8.5-20.7-19.7l0-125.1c-.3-11.1 8.6-20.4 19.7-20.7l63.2 0 0 29.5zM358.5 317.8c-13.2 30.7-36.8 24.6-47.4 0l-38.5-144.8 32.6 0 29.7 113.7 29.6-113.7 32.6 0-38.5 144.8z",
            }
        }
    }
}
