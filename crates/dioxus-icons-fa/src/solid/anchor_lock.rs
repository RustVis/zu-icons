// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AnchorLockProps {
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

pub fn AnchorLock(props: AnchorLockProps) -> Element {
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
        d: "M320.5 96a32 32 0 1 1 -64 0 32 32 0 1 1 64 0zm-32-96c-53 0-96 43-96 96 0 41.8 26.7 77.4 64 90.5l0 257.9c-62.9-14.3-110.2-69.7-111.9-136.5l16.1 14.1c10 8.7 25.1 7.7 33.9-2.3s7.7-25.1-2.3-33.9l-64-56c-9-7.9-22.6-7.9-31.6 0l-64 56c-10 8.7-11 23.9-2.3 33.9s23.9 11 33.9 2.3l16.2-14.2c2.1 113.1 94.4 204.1 208 204.1 28.3 0 55.4-5.7 80-15.9l0-.2 0-72.2c-14.4 9.6-30.6 16.7-48 20.7l0-257.9c37.3-13.2 64-48.7 64-90.5 0-53-43-96-96-96zm272 304.1l0 47.9-64 0 0-47.9c0-17.7 14.3-32 32-32s32 14.3 32 32zM416.5 400l0 96c0 26.5 21.5 48 48 48l128 0c26.5 0 48-21.5 48-48l0-96c0-20.9-13.4-38.7-32-45.3l0-50.6c0-44.2-35.8-80-80-80s-80 35.8-80 80l0 50.6c-18.6 6.6-32 24.4-32 45.3z",
            }
        }
    }
}
