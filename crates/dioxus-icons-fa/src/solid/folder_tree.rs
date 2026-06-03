// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FolderTreeProps {
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

pub fn FolderTree(props: FolderTreeProps) -> Element {
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
        d: "M48 24C48 10.7 37.3 0 24 0S0 10.7 0 24L0 392c0 30.9 25.1 56 56 56l184 0 0-48-184 0c-4.4 0-8-3.6-8-8l0-232 192 0 0-48-192 0 0-88zM336 224l192 0c26.5 0 48-21.5 48-48l0-96c0-26.5-21.5-48-48-48l-82.7 0c-8.5 0-16.6-3.4-22.6-9.4l-8.6-8.6c-9-9-21.2-14.1-33.9-14.1L336 0c-26.5 0-48 21.5-48 48l0 128c0 26.5 21.5 48 48 48zm0 288l192 0c26.5 0 48-21.5 48-48l0-96c0-26.5-21.5-48-48-48l-82.7 0c-8.5 0-16.6-3.4-22.6-9.4l-8.6-8.6c-9-9-21.2-14.1-33.9-14.1L336 288c-26.5 0-48 21.5-48 48l0 128c0 26.5 21.5 48 48 48z",
            }
        }
    }
}
