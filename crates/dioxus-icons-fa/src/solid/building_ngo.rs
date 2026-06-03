// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BuildingNgoProps {
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

pub fn BuildingNgo(props: BuildingNgoProps) -> Element {
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
        d: "M128 0C92.7 0 64 28.7 64 64l0 384c0 35.3 28.7 64 64 64l48 0 0-112c0-35.3 28.7-64 64-64l208 0 0-272c0-35.3-28.7-64-64-64L128 0zm32 112c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32zM304 96l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32c0-8.8 7.2-16 16-16zM160 240c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32zm144-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32c0-8.8 7.2-16 16-16zM520 380c-24.3 0-44 19.7-44 44l0 80c0 24.3 19.7 44 44 44l16 0c24.3 0 44-19.7 44-44l0-80c0-24.3-19.7-44-44-44l-16 0zm-4 44c0-2.2 1.8-4 4-4l16 0c2.2 0 4 1.8 4 4l0 80c0 2.2-1.8 4-4 4l-16 0c-2.2 0-4-1.8-4-4l0-80zm-168 0l0 80c0 24.3 19.7 44 44 44l16 0c24.3 0 44-19.7 44-44l0-24c0-11-9-20-20-20l-8 0c-11 0-20 9-20 20 0 6.5 3.1 12.4 8 16l0 8c0 2.2-1.8 4-4 4l-16 0c-2.2 0-4-1.8-4-4l0-80c0-2.2 1.8-4 4-4l20.4 0c1.9 9.1 9.9 16 19.6 16 11 0 20-9 20-20 0-19.9-16.1-36-36-36l-24 0c-24.3 0-44 19.7-44 44zm-90.1-32.9c-4.1-8.3-13.5-12.7-22.5-10.5S220 390.7 220 400l0 128c0 11 9 20 20 20s20-9 20-20l0-43.3 26.1 52.2c4.1 8.3 13.5 12.7 22.5 10.5S324 537.3 324 528l0-128c0-11-9-20-20-20s-20 9-20 20l0 43.3-26.1-52.2z",
            }
        }
    }
}
