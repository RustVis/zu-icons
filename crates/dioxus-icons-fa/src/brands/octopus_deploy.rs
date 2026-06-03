// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct OctopusDeployProps {
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

pub fn OctopusDeploy(props: OctopusDeployProps) -> Element {
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
        d: "M455.7 349.2c-45.9-39.1-36.7-77.9-16.1-128.1 35.7-87-23.5-186.9-109.6-212.8-92.9-27.9-195.7 16-230.3 108.8-8.7 23.5-12.5 48.5-11 73.5 1.7 29.5 14.7 53 24.1 80.3 17.2 50.2-28.1 92.7-66.7 117.6-46.8 30.2-36.3 39.9-8.4 41.9 23.4 1.7 44.5-4.5 65.3-15 9.2-4.6 40.7-18.9 45.1-28.6-12.2 26.6-37 72.7-21.5 102.1 19.1 36.2 67.1-31.8 76.7-45.8 8.6-12.6 43-81.3 63.6-46.9 18.9 31.4 8.6 76.4 35.7 104.6 32.9 34.2 51.2-18.3 51.4-44.2 .2-16.4-6.1-95.9 29.9-59.9 21.4 21.4 52.9 71.2 88.6 67 38.7-4.5-22.1-68-28.3-78.7 5.4 4.3 53.7 34.1 53.8 9.5 .1-18.8-30.1-34.7-42.5-45.3z",
            }
        }
    }
}
