// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SectionProps {
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

    #[props(default = Some("0 0 256 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Section(props: SectionProps) -> Element {
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
        d: "M110 0C49.2 0 0 49.2 0 110 0 133.7 7.6 155.8 20.5 174 7.6 192 0 214.1 0 238 0 291.7 38.9 337.6 91.9 346.4l61.7 10.3c22.2 3.7 38.4 22.9 38.4 45.3 0 25.4-20.6 46-46 46l-98 0c-17.7 0-32 14.3-32 32s14.3 32 32 32l98 0c60.7 0 110-49.2 110-110 0-23.7-7.6-45.9-20.5-64 12.9-18 20.5-40.1 20.5-64 0-53.8-38.9-99.6-91.9-108.5l-61.7-10.3C80.2 151.6 64 132.4 64 110 64 84.6 84.6 64 110 64l98 0c17.7 0 32-14.3 32-32S225.7 0 208 0L110 0zm74.7 299.1c-6.5-2.4-13.4-4.3-20.5-5.5l-61.7-10.3c-22.2-3.7-38.4-22.9-38.4-45.3 0-9.2 2.7-17.8 7.4-25 6.5 2.4 13.4 4.3 20.5 5.5l61.7 10.3c22.2 3.7 38.4 22.9 38.4 45.3 0 9.2-2.7 17.8-7.4 25z",
            }
        }
    }
}
