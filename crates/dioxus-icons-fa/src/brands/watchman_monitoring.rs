// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct WatchmanMonitoringProps {
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

pub fn WatchmanMonitoring(props: WatchmanMonitoringProps) -> Element {
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
        d: "M256 16a240 240 0 1 0 0 480 240 240 0 1 0 0-480zM121.7 429.1c-51.6-40.2-84.9-102.8-84.9-173.1 0-21.7 3.2-43.3 9.6-64.1l102.9-17.9-.1 11-13.9 2s-.1 12.5-.1 19.5c0 2 .4 4 1.2 5.8s2.1 3.4 3.7 4.6l9.5 7.4-27.7 204.9zM227.4 145.9l8.5-7.6s6.9-5.4-.1-9.3c-7.2-4-39.5-34.5-39.5-34.5-5.3-5.5-8.3-7.3-15.5 0 0 0-32.3 30.5-39.5 34.5-7.1 4-.1 9.3-.1 9.3l8.5 7.6 0 4.4-73.5-19.2c39.6-56.9 105.5-94.3 180-94.3 31.3 0 62.2 6.7 90.6 19.6s53.8 31.8 74.3 55.4l-193.5 37.7 0-3.6zm34.1 329.3l-33.9-250.9 9.5-7.4c1.6-1.2 2.8-2.8 3.7-4.6s1.3-3.8 1.2-5.8c0-7-.1-19.5-.1-19.5l-13.9-2-.1-10.5 241.7 31.4c3.9 16.4 5.8 33.3 5.8 50.1 0 119.1-95.4 216.2-213.8 219.1z",
            }
        }
    }
}
