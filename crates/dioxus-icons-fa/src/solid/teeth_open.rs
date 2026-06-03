// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TeethOpenProps {
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

pub fn TeethOpen(props: TeethOpenProps) -> Element {
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
        d: "M64 32C28.7 32 0 60.7 0 96l0 80c0 35.3 28.7 64 64 64l384 0c35.3 0 64-28.7 64-64l0-80c0-35.3-28.7-64-64-64L64 32zm0 272c-35.3 0-64 28.7-64 64l0 48c0 35.3 28.7 64 64 64l384 0c35.3 0 64-28.7 64-64l0-48c0-35.3-28.7-64-64-64L64 304zm80-160c0-26.5 21.5-48 48-48s48 21.5 48 48l0 24c0 13.3-10.7 24-24 24l-48 0c-13.3 0-24-10.7-24-24l0-24zm128 0c0-26.5 21.5-48 48-48s48 21.5 48 48l0 24c0 13.3-10.7 24-24 24l-48 0c-13.3 0-24-10.7-24-24l0-24zM80 112c17.7 0 32 14.3 32 32l0 24c0 13.3-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24l0-24c0-17.7 14.3-32 32-32zm320 32c0-17.7 14.3-32 32-32s32 14.3 32 32l0 24c0 13.3-10.7 24-24 24l-16 0c-13.3 0-24-10.7-24-24l0-24zM48 384l0-16c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 16c0 17.7-14.3 32-32 32s-32-14.3-32-32zm144 48c-26.5 0-48-21.5-48-48l0-16c0-8.8 7.2-16 16-16l64 0c8.8 0 16 7.2 16 16l0 16c0 26.5-21.5 48-48 48zm128 0c-26.5 0-48-21.5-48-48l0-16c0-8.8 7.2-16 16-16l64 0c8.8 0 16 7.2 16 16l0 16c0 26.5-21.5 48-48 48zm112-16c-17.7 0-32-14.3-32-32l0-16c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 16c0 17.7-14.3 32-32 32z",
            }
        }
    }
}
