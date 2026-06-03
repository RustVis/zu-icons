// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct VirusSlashProps {
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

pub fn VirusSlash(props: VirusSlashProps) -> Element {
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
        d: "M41-24.9c-9.4-9.4-24.6-9.4-33.9 0S-2.3-.3 7 9.1l528 528c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9l-88.6-88.6c1.7-12.1-2-24.8-11.3-34-31.2-31.2-9.1-84.5 35-84.5 22.1 0 40-17.9 40-40s-17.9-40-40-40c-44.1 0-66.1-53.3-35-84.5 15.6-15.6 15.6-40.9 0-56.6s-40.9-15.6-56.6 0c-31.2 31.2-84.5 9.1-84.5-35 0-22.1-17.9-40-40-40s-40 17.9-40 40c0 44.1-53.3 66.1-84.5 35-9.3-9.3-22-13-34-11.3L41-24.9zM72 216c-22.1 0-40 17.9-40 40s17.9 40 40 40c44.1 0 66.1 53.3 35 84.5-15.6 15.6-15.6 40.9 0 56.6s40.9 15.6 56.6 0c31.2-31.2 84.5-9.1 84.5 35 0 22.1 17.9 40 40 40s40-17.9 40-40c0-21.4 12.6-37.6 29.1-45.1l-240-240C109.6 203.4 93.4 216 72 216z",
            }
        }
    }
}
