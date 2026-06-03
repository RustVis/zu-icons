// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LaptopFileProps {
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

pub fn LaptopFile(props: LaptopFileProps) -> Element {
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
        d: "M64 64C64 28.7 92.7 0 128 0L416 0c35.3 0 64 28.7 64 64l0 48-64 0 0-48-288 0 0 192 112 0 0 96-163.2 0C34.4 352 0 317.6 0 275.2 0 264.6 8.6 256 19.2 256L64 256 64 64zM529.9 257.9c9 9 14.1 21.2 14.1 33.9L544 464c0 26.5-21.5 48-48 48l-160 0c-26.5 0-48-21.5-48-48l0-256c0-26.5 21.5-48 48-48l76.1 0c12.7 0 24.9 5.1 33.9 14.1 20 20 47.9 47.9 83.9 83.9zM416 272c0 8.8 7.2 16 16 16l60.1 0-76.1-76.1 0 60.1z",
            }
        }
    }
}
