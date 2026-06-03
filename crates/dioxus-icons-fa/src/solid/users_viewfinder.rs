// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UsersViewfinderProps {
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

pub fn UsersViewfinder(props: UsersViewfinderProps) -> Element {
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
        d: "M64 0C28.7 0 0 28.7 0 64l0 72c0 13.3 10.7 24 24 24s24-10.7 24-24l0-72c0-8.8 7.2-16 16-16l72 0c13.3 0 24-10.7 24-24S149.3 0 136 0L64 0zM440 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l72 0c8.8 0 16 7.2 16 16l0 72c0 13.3 10.7 24 24 24s24-10.7 24-24l0-72c0-35.3-28.7-64-64-64L440 0zM48 376c0-13.3-10.7-24-24-24S0 362.7 0 376l0 72c0 35.3 28.7 64 64 64l72 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-72 0c-8.8 0-16-7.2-16-16l0-72zm528 0c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 72c0 8.8-7.2 16-16 16l-72 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l72 0c35.3 0 64-28.7 64-64l0-72zM288 216a56 56 0 1 0 0-112 56 56 0 1 0 0 112zm0 40c-53 0-96 43-96 96l0 24c0 13.3 10.7 24 24 24l144 0c13.3 0 24-10.7 24-24l0-24c0-53-43-96-96-96zm192-64a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM168 272.3c-49.3 4.1-88 45.3-88 95.7l0 10.7c0 11.8 9.6 21.3 21.3 21.3l46.8 0c-2.7-7.5-4.1-15.6-4.1-24l0-24c0-29.5 8.8-56.9 24-79.7zM427.9 400l46.8 0c11.8 0 21.3-9.6 21.3-21.3l0-10.7c0-50.3-38.7-91.6-88-95.7 15.2 22.8 24 50.2 24 79.7l0 24c0 8.4-1.4 16.5-4.1 24zM192 192a48 48 0 1 0 -96 0 48 48 0 1 0 96 0z",
            }
        }
    }
}
