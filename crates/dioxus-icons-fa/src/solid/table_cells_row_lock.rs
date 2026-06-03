// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableCellsRowLockProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn TableCellsRowLock(props: TableCellsRowLockProps) -> Element {
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
        d: "M256 288l0-64 153.3 0c17.1-42.2 56-73.2 102.7-79l0-49c0-35.3-28.7-64-64-64L128 32C92.7 32 64 60.7 64 96l0 320c0 35.3 28.7 64 64 64l241.3 0c-.9-5.2-1.3-10.6-1.3-16.1l0-47.9-112 0 0-64 113.3 0c3.7-22 14.8-41.4 30.7-55.6l0-8.4-144 0zm0-192l192 0 0 64-192 0 0-64zM560 272.1l0 47.9-64 0 0-47.9c0-17.7 14.3-32 32-32s32 14.3 32 32zM416 368l0 96c0 26.5 21.5 48 48 48l128 0c26.5 0 48-21.5 48-48l0-96c0-20.9-13.4-38.7-32-45.3l0-50.6c0-44.2-35.8-80-80-80s-80 35.8-80 80l0 50.6c-18.6 6.6-32 24.4-32 45.3z",
            }
        }
    }
}
