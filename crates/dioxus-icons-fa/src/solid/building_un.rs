// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BuildingUnProps {
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

pub fn BuildingUn(props: BuildingUnProps) -> Element {
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
        d: "M32 64C32 28.7 60.7 0 96 0L352 0c35.3 0 64 28.7 64 64l0 272-112 0c-20.9 0-39.5 10.1-51.2 25.6-8-6-18-9.6-28.8-9.6-26.5 0-48 21.5-48 48l0 64 64 0 0 48-144 0c-35.3 0-64-28.7-64-64L32 64zM144 96c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zm112 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zM144 224c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zm112 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zM427.4 380.5c9-2.1 18.3 2.2 22.5 10.5l26.1 52.2 0-43.3c0-11 9-20 20-20s20 9 20 20l0 128c0 9.3-6.4 17.3-15.4 19.5s-18.3-2.2-22.5-10.5L452 484.7 452 528c0 11-9 20-20 20s-20-9-20-20l0-128c0-9.3 6.4-17.3 15.4-19.5zM324 400l0 96c0 6.6 5.4 12 12 12s12-5.4 12-12l0-96c0-11 9-20 20-20s20 9 20 20l0 96c0 28.7-23.3 52-52 52s-52-23.3-52-52l0-96c0-11 9-20 20-20s20 9 20 20z",
            }
        }
    }
}
