// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DiggProps {
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

pub fn Digg(props: DiggProps) -> Element {
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
        d: "M81.7 172.3l-81.7 0 0 174.4 132.7 0 0-250.7-51 0 0 76.3zm0 133.4l-30.8 0 0-92.3 30.8 0 0 92.3zM378.9 172.3l0 174.4 81.8 0 0 28.5-81.8 0 0 40.8 133.1 0 0-243.7-133.1 0zm81.8 133.4l-30.8 0 0-92.3 30.8 0 0 92.3zm-235.6 41l82.1 0 0 28.5-82.1 0 0 40.8 133.3 0 0-243.7-133.3 0 0 174.4zm51.2-133.3l30.8 0 0 92.3-30.8 0 0-92.3zM153.3 96l51.3 0 0 51-51.3 0 0-51zm0 76.3l51.3 0 0 174.4-51.3 0 0-174.4z",
            }
        }
    }
}
