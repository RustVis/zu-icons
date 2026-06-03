// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MedrtProps {
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

pub fn Medrt(props: MedrtProps) -> Element {
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
        d: "M129.7 256c0 121.8 83.9 222.8 193.5 241.1-18.7 4.5-38.2 6.9-58.2 6.9-137.6 0-249-111-249-248S127.4 8 264.9 8c20.1 0 39.6 2.4 58.2 6.9-109.6 18.3-193.4 119.3-193.4 241.1zM427.1 356.3c-77.7 55.4-179.6 47.5-240.4-14.6 5.5 14.1 12.7 27.7 21.7 40.5 61.6 88.2 182.4 109.3 269.7 47s108.1-184.3 46.5-272.6c-9-12.9-19.3-24.3-30.5-34.2 37.4 78.8 10.7 178.5-67 233.9zm-218.8-244c-1.4 1-2.7 2.1-4 3.1 64.3-17.8 135.9 4 178.9 60.5 35.7 47 42.9 106.6 24.4 158 56.7-56.2 67.6-142.1 22.3-201.8-50-65.5-149.1-74.4-221.6-19.8zM312 224c-4.4 0-8-3.6-8-8l0-40c0-4.4-3.6-8-8-8l-48 0c-4.4 0-8 3.6-8 8l0 40c0 4.4-3.6 8-8 8l-40 0c-4.4 0-8 3.6-8 8l0 48c0 4.4 3.6 8 8 8l40 0c4.4 0 8 3.6 8 8l0 40c0 4.4 3.6 8 8 8l48 0c4.4 0 8-3.6 8-8l0-40c0-4.4 3.6-8 8-8l40 0c4.4 0 8-3.6 8-8l0-48c0-4.4-3.6-8-8-8l-40 0z",
            }
        }
    }
}
