// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DiagramPredecessorProps {
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

pub fn DiagramPredecessor(props: DiagramPredecessorProps) -> Element {
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
        d: "M289.2 137.9c2.5-6 8.3-9.9 14.8-9.9l40 0 0-24c0-13.3-10.7-24-24-24l-98 0c1.3 5.1 2 10.5 2 16l0 64c0 35.3-28.7 64-64 64l-96 0c-35.3 0-64-28.7-64-64L0 96C0 60.7 28.7 32 64 32l256 0c39.8 0 72 32.2 72 72l0 24 40 0c6.5 0 12.3 3.9 14.8 9.9s1.1 12.9-3.5 17.4l-64 64c-6.2 6.2-16.4 6.2-22.6 0l-64-64c-4.6-4.6-5.9-11.5-3.5-17.4zM384 352l-320 0 0 64 320 0 0-64zm64 64c0 35.3-28.7 64-64 64L64 480c-35.3 0-64-28.7-64-64l0-64c0-35.3 28.7-64 64-64l320 0c35.3 0 64 28.7 64 64l0 64z",
            }
        }
    }
}
