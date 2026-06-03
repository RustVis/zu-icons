// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CentosProps {
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

pub fn Centos(props: CentosProps) -> Element {
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
        d: "M289.6 97.5l31.6 31.7-76.3 76.5 0-108.2 44.7 0zM127.2 129.2l76.3 76.5 0-108.2-44.7 0-31.6 31.7zm41.5-41.6l44.7 0 0 127.9 10.8 10.8 10.8-10.8 0-127.9 44.7 0-55.5-55.6-55.5 55.6zm26.2 168.1l-10.8-10.8-128.6 0 0-44.8-55.5 55.6 55.5 55.6 0-44.8 128.6 0 10.8-10.8zM274.2 235l107.9 0 0-44.8-31.6-31.7-76.3 76.5zm173.3 20.7l-55.5-55.6 0 44.8-127.7 0-10.8 10.8 10.8 10.8 127.7 0 0 44.8 55.5-55.6zM65.4 176.2l32.5-31.7 90.3 90.5 15.3 0 0-15.3-90.3-90.5 31.6-31.7-79.4 0 0 78.7zM382.1 97.5l-78.5 0 31.6 31.7-90.3 90.5 0 15.3 15.3 0 90.3-90.5 31.6 31.7 0-78.7zM203.5 413.9l0-108.1-76.3 76.5 31.6 31.7 44.7 0 0-.1zM65.4 235l108.8 0-76.3-76.5-32.5 31.7 0 44.8zM382.1 335.2l-31.6 31.7-90.3-90.5-15.3 0 0 15.3 90.3 90.5-31.6 31.7 78.5 0 0-78.7zm0-58.8l-107.9 0 76.3 76.5 31.6-31.7 0-44.8zM321.2 382.2l-76.3-76.5 0 108.1 44.7 0 31.6-31.6zM97.9 352.9l76.3-76.5-108.8 0 0 44.8 32.5 31.7zm181.8 70.9l-44.7 0 0-127.9-10.8-10.8-10.8 10.8 0 127.9-44.7 0 55.5 55.6 55.5-55.6zM113.2 382.2l90.3-90.5 0-15.3-15.3 0-90.3 90.5-32.5-31.7 0 78.7 79.4 0-31.6-31.7z",
            }
        }
    }
}
