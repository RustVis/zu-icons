// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct VirusProps {
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

pub fn Virus(props: VirusProps) -> Element {
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
        d: "M296 40c0-22.1-17.9-40-40-40s-40 17.9-40 40c0 44.1-53.3 66.1-84.5 35-15.6-15.6-40.9-15.6-56.6 0s-15.6 40.9 0 56.6c31.2 31.2 9.1 84.5-35 84.5-22.1 0-40 17.9-40 40s17.9 40 40 40c44.1 0 66.1 53.3 35 84.5-15.6 15.6-15.6 40.9 0 56.6s40.9 15.6 56.6 0c31.2-31.2 84.5-9.1 84.5 35 0 22.1 17.9 40 40 40s40-17.9 40-40c0-44.1 53.3-66.1 84.5-35 15.6 15.6 40.9 15.6 56.6 0s15.6-40.9 0-56.6c-31.2-31.2-9.1-84.5 35-84.5 22.1 0 40-17.9 40-40s-17.9-40-40-40c-44.1 0-66.1-53.3-35-84.5 15.6-15.6 15.6-40.9 0-56.6s-40.9-15.6-56.6 0C349.3 106.1 296 84.1 296 40zM160 224a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zm160 32a32 32 0 1 1 0 64 32 32 0 1 1 0-64z",
            }
        }
    }
}
