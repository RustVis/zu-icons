// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LandMineOnProps {
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

    #[props(default = Some("0 0 576 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn LandMineOn(props: LandMineOnProps) -> Element {
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
        d: "M312 0l0 128c0 13.3-10.7 24-24 24s-24-10.7-24-24L264 0c0-13.3 10.7-24 24-24s24 10.7 24 24zM160 288c0-17.7 14.3-32 32-32l192 0c17.7 0 32 14.3 32 32l0 32 80 0c26.5 0 48 21.5 48 48l0 96c0 26.5-21.5 48-48 48L80 512c-26.5 0-48-21.5-48-48l0-96c0-26.5 21.5-48 48-48l80 0 0-32zM12 114.7c7.4-11 22.3-14 33.3-6.7l96 64c11 7.4 14 22.3 6.7 33.3s-22.3 14-33.3 6.7l-96-64c-11-7.4-14-22.3-6.7-33.3zM530.7 108c11-7.4 25.9-4.4 33.3 6.7s4.4 25.9-6.7 33.3l-96 64c-11 7.4-25.9 4.4-33.3-6.7s-4.4-25.9 6.7-33.3l96-64z",
            }
        }
    }
}
