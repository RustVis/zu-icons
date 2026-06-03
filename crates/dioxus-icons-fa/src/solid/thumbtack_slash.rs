// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ThumbtackSlashProps {
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

pub fn ThumbtackSlash(props: ThumbtackSlashProps) -> Element {
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
        d: "M41-24.9c-9.4-9.4-24.6-9.4-33.9 0S-2.3-.3 7 9.1l528 528c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9L417.8 352 448 352c10 0 19.5-4.7 25.5-12.7s8-18.4 5.2-28.1L475 297.8c-12.4-43.3-41-78.5-78.2-99.7L386.5 64 416 64c17.7 0 32-14.3 32-32S433.7 0 416 0L160 0c-7.4 0-14.1 2.5-19.5 6.6L190.1 56.3 185.3 119.4 41-24.9zM282.2 352L149.7 219.6c-22.7 20.5-39.8 47.4-48.7 78.3l-3.8 13.4c-2.8 9.7-.8 20 5.2 28.1S118 352 128 352l154.2 0zM256 512c0 17.7 14.3 32 32 32s32-14.3 32-32l0-112-64 0 0 112z",
            }
        }
    }
}
