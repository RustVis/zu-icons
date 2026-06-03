// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SkyatlasProps {
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

pub fn Skyatlas(props: SkyatlasProps) -> Element {
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
        d: "M640 329.3c0 65.9-52.5 114.4-117.5 114.4-165.9 0-196.6-249.7-359.7-249.7-146.9 0-147.1 212.2 5.6 212.2 42.5 0 90.9-17.8 125.3-42.5 5.6-4.1 16.9-16.3 22.8-16.3s10.9 5 10.9 10.9c0 7.8-13.1 19.1-18.7 24.1-40.9 35.6-100.3 61.2-154.7 61.2-83.4 .1-154-59-154-144.9S67.5 149.6 152.8 149.6c185.3 0 222.5 245.9 361.9 245.9 99.9 0 94.8-139.7 3.4-139.7-17.5 0-35 11.6-46.9 11.6-8.4 0-15.9-7.2-15.9-15.6 0-11.6 5.3-23.7 5.3-36.3 0-66.6-50.9-114.7-116.9-114.7-53.1 0-80 36.9-88.8 36.9-6.2 0-11.2-5-11.2-11.2 0-5.6 4.1-10.3 7.8-14.4 25.3-28.8 64.7-43.7 102.8-43.7 79.4 0 139.1 58.4 139.1 137.8 0 6.9-.3 13.7-1.2 20.6 11.9-3.1 24.1-4.7 35.9-4.7 60.7 0 111.9 45.3 111.9 107.2z",
            }
        }
    }
}
