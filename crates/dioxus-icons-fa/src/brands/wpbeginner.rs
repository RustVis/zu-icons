// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct WpbeginnerProps {
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

pub fn Wpbeginner(props: WpbeginnerProps) -> Element {
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
        d: "M463.2 322.4c56.2 64.3 4.2 157.6-91.9 157.6-39.6 0-78.8-17.7-100.1-50-6.9 .4-22.7 .4-29.6 0-21.4 32.4-60.6 50-100.1 50-95.5 0-148.3-93-91.9-157.6-79.1-131.9 31.3-290.4 206.8-290.4 175.6 0 285.9 158.6 206.8 290.4zm-339.6-83l41.5 0 0-58.1-41.5 0 0 58.1zm217.2 86.1l0-23.8c-60.5 20.9-132.4 9.2-187.6-34l.2 24.9c51.1 46.4 131.7 57.9 187.3 32.9zM190 239.4l166.1 0 0-58.1-166.1 0 0 58.1z",
            }
        }
    }
}
