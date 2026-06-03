// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ZoomProps {
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

pub fn Zoom(props: ZoomProps) -> Element {
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
        d: "M134.2 326.6l-114.4 0c-8 0-15.2-4.8-18.3-12.2S.1 298.6 5.8 292.9l79.3-79.3-56.8 0C12.7 213.6 0 201 0 185.4l105.5 0c8 0 15.2 4.8 18.3 12.2s1.4 15.9-4.3 21.6l-79.3 79.3 65.7 0c15.6 0 28.3 12.6 28.3 28.3zM640 238.3c0-30.4-24.7-55.1-55.1-55.1-16.2 0-30.9 7.1-41 18.3-10.1-11.2-24.7-18.3-41-18.3-30.4 0-55.1 24.7-55.1 55.1l0 88.3c15.6 0 28.3-12.7 28.3-28.3l0-60c0-14.8 12-26.8 26.8-26.8s26.8 12 26.8 26.8l0 60c0 15.6 12.6 28.3 28.3 28.3l0-88.3c0-14.8 12-26.8 26.8-26.8s26.8 12 26.8 26.8l0 60c0 15.6 12.6 28.3 28.3 28.3l0-88.3zM288.2 256a72.8 72.8 0 1 1 145.5 0 72.8 72.8 0 1 1 -145.5 0zm117.3 0a44.5 44.5 0 1 0 -89 0 44.5 44.5 0 1 0 89 0zm-274.1 0a72.8 72.8 0 1 1 145.5 0 72.8 72.8 0 1 1 -145.5 0zm117.3 0a44.5 44.5 0 1 0 -89 0 44.5 44.5 0 1 0 89 0z",
            }
        }
    }
}
