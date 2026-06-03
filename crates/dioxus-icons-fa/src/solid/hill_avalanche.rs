// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HillAvalancheProps {
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

pub fn HillAvalanche(props: HillAvalancheProps) -> Element {
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
        d: "M440.1 401.9c34.2 23.1 81.1 19.5 111.4-10.8 34.4-34.4 34.4-90.1 0-124.5-27.8-27.8-69.5-33.1-102.6-16-11.8 6.1-16.4 20.6-10.3 32.3s20.6 16.4 32.3 10.3c15.1-7.8 34-5.3 46.6 7.3 15.6 15.6 15.6 40.9 0 56.6s-40.9 15.6-56.6 0l-81.7-81.7c22.3-14.2 37.1-39.1 37.1-67.5 0-33.9-21.1-62.9-50.9-74.5 1.9-6.8 2.9-14 2.9-21.5 0-44.2-35.8-80-80-80-27.3 0-51.5 13.7-65.9 34.6-5.8-20-24.2-34.6-46.1-34.6-26.5 0-48 21.5-48 48 0 4 .5 7.9 1.4 11.6L440.1 401.9zM480.4 64a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm0 128a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM68.7 87C43.5 61.8 .4 79.7 .4 115.3L.4 432c0 44.2 35.8 80 80 80l316.7 0c35.6 0 53.5-43.1 28.3-68.3L68.7 87z",
            }
        }
    }
}
