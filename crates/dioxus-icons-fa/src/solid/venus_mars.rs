// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct VenusMarsProps {
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

pub fn VenusMars(props: VenusMarsProps) -> Element {
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
        d: "M480-64c-17.7 0-32 14.3-32 32S462.3 0 480 0L530.7 0 474 56.7c-26.3-15.7-57.1-24.7-90-24.7-35.4 0-68.4 10.5-96 28.5-27.6-18-60.6-28.5-96-28.5-97.2 0-176 78.8-176 176 0 86.3 62.1 158.1 144 173.1l0 34.9-32 0c-17.7 0-32 14.3-32 32s14.3 32 32 32l32 0 0 32c0 17.7 14.3 32 32 32s32-14.3 32-32l0-32 32 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-32 0 0-34.9c23.3-4.3 44.9-13.1 64-25.6 27.6 18 60.6 28.5 96 28.5 97.2 0 176-78.8 176-176 0-41.1-14.1-79-37.8-109L576 45.3 576 96c0 17.7 14.3 32 32 32s32-14.3 32-32l0-128c0-17.7-14.3-32-32-32L480-64zM336 309.2c20.2-28.6 32-63.5 32-101.2s-11.8-72.6-32-101.2c14.6-6.9 30.8-10.8 48-10.8 61.9 0 112 50.1 112 112S445.9 320 384 320c-17.2 0-33.5-3.9-48-10.8zM288 150.3c10.2 16.9 16 36.6 16 57.7s-5.8 40.9-16 57.7c-10.2-16.9-16-36.6-16-57.7s5.8-40.9 16-57.7zm-48-43.5c-20.2 28.6-32 63.5-32 101.2s11.8 72.6 32 101.2c-14.5 6.9-30.8 10.8-48 10.8-61.9 0-112-50.1-112-112S130.1 96 192 96c17.2 0 33.5 3.9 48 10.8z",
            }
        }
    }
}
