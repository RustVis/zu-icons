// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ChromecastProps {
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

pub fn Chromecast(props: ChromecastProps) -> Element {
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
        d: "M448 64L64.2 64c-23.6 0-42.7 19.1-42.7 42.7l0 63.9 42.7 0 0-63.9 383.8 0 0 298.6-149.2 0 0 42.7 149.4 0c23.6 0 42.7-19.1 42.7-42.7l0-298.6C490.9 83.1 471.6 64 448 64zM21.5 383.6l0 63.9 63.9 0c0-35.3-28.6-63.9-63.9-63.9zm0-85l0 42.4c58.9 0 106.6 48.1 106.6 107l42.7 0c.1-82.4-66.9-149.3-149.3-149.4zM213.6 448l42.7 0C255.8 318.5 151 213.7 21.5 213.4l0 42.4c106-.2 192 86.2 192.1 192.2z",
            }
        }
    }
}
