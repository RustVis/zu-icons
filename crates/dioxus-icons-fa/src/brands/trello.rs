// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TrelloProps {
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

pub fn Trello(props: TrelloProps) -> Element {
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
        d: "M392.3 32L56.1 32c-31 0-56.1 25.1-56.1 56-.1 0 0-4 0 336 0 30.9 25.1 56 56 56l336.2 0c30.8-.2 55.7-25.2 55.7-56l0-336c.1-30.8-24.8-55.8-55.6-56zM197 371.3c-.2 14.7-12.1 26.6-26.9 26.6l-82.7 0c-14.8 .1-26.9-11.8-27-26.6l0-254.2c0-14.8 12-26.9 26.9-26.9l82.9 0c14.8 0 26.9 12 26.9 26.9l0 254.2-.1 0zm193.1-112c0 14.8-12 26.9-26.9 26.9l-81 0c-14.8 0-26.9-12-26.9-26.9l0-142.1c0-14.8 12-26.9 26.8-26.9l81.1 0c14.8 0 26.9 12 26.9 26.9l0 142.1z",
            }
        }
    }
}
