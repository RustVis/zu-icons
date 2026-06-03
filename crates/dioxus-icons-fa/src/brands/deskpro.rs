// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DeskproProps {
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

pub fn Deskpro(props: DeskproProps) -> Element {
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
        d: "M122 406.2l84 0c23.1 0 43.9-3.5 62.6-10.7 18.7-7.1 34.7-17.1 48-30.1 13.3-13.4 23.6-29.2 30.7-47.5 3.9-9.6 6.8-19.8 8.6-30.6l73.7 0c-2.5 21.3-7.7 41.3-15.5 60-11.1 27.2-26.9 50.8-47.3 70.9-20 19.6-44.2 34.8-72.6 45.4s-60 16-94.7 16l-149.8 .2 0-192.4 72.3-.2 0 118.8zM199.4 32c34.7 0 66.2 5.4 94.7 16 28.5 10.7 52.9 26.1 73.3 46.2 20.4 19.6 36.2 43.2 47.3 70.9 7.5 18.3 12.4 38.1 14.9 59.3l-73.7-.1c-1.9-10.5-4.7-20.5-8.6-29.9-7.1-18.7-17.3-34.5-30.7-47.5-13.3-13.4-29.3-23.6-48-30.8-18.6-7.1-39.5-10.7-62.7-10.7l-84 0 0 118.8-72.2 .2 0-192.3 149.7-.1z",
            }
        }
    }
}
