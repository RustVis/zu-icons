// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PersonHalfDressProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn PersonHalfDress(props: PersonHalfDressProps) -> Element {
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
        d: "M143.4-3.9c9.7-16.8 27.8-28.1 48.6-28.1 30.9 0 56 25.1 56 56 0 25.4-16.9 46.8-40 53.7l0 0c-5.1 1.5-10.4 2.3-16 2.3-30.9 0-56-25.1-56-56l0 0c0-10.1 2.7-19.7 7.4-27.9zM318.3 299.1L272 236.7 272 512c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-160 0-.4 0-238.6c36.2 4.4 69.2 23.4 91.2 53l70.5 95c10.5 14.2 7.6 34.2-6.6 44.8s-34.2 7.6-44.8-6.6zM176 113l0 399c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-128-25.8 0c-10.9 0-18.6-10.7-15.2-21.1l43-129-48.3 65.1c-10.5 14.2-30.6 17.2-44.8 6.6s-17.2-30.6-6.6-44.8l70.5-95c22-29.6 55.1-48.6 91.2-53z",
            }
        }
    }
}
