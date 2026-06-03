// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PanoramaProps {
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

pub fn Panorama(props: PanoramaProps) -> Element {
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
        d: "M43.9 48C19.7 48 0 67.7 0 91.9L0 420.1c0 24.3 19.7 43.9 43.9 43.9 5.5 0 10.7-1 15.7-2.9 12.9-4.9 103.4-37.1 228.4-37.1s215.5 32.3 228.4 37.1c5 1.9 10.2 2.9 15.7 2.9 24.3 0 43.9-19.7 43.9-43.9l0-328.2c0-24.3-19.7-43.9-43.9-43.9-5.5 0-10.7 1-15.7 2.9-12.9 4.9-103.4 37.1-228.4 37.1S72.5 55.7 59.6 50.9C54.6 49 49.4 48 43.9 48zM72 176a40 40 0 1 1 80 0 40 40 0 1 1 -80 0zm264.1-16c7.5 0 14.6 3.6 19.1 9.6L479.7 336.2c5.9 7.9 6.4 18.5 1.3 26.9s-14.8 12.8-24.5 11.1c-45.8-7.8-103.3-14.2-168.4-14.2-65.6 0-123.4 6.5-169.3 14.4-9.8 1.7-19.7-2.9-24.7-11.5s-4.3-19.4 1.9-27.2L165.3 249c4.6-5.7 11.5-9 18.7-9s14.2 3.3 18.7 9l27.5 34.4 86.7-113.9c4.6-6 11.7-9.5 19.2-9.5z",
            }
        }
    }
}
