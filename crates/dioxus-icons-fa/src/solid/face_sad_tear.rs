// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FaceSadTearProps {
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

    #[props(default = Some("0 0 512 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn FaceSadTear(props: FaceSadTearProps) -> Element {
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
        d: "M0 256a256 256 0 1 1 512 0 256 256 0 1 1 -512 0zm271 98c5.5-1.3 11.1-2 17-2 24.1 0 45.4 11.8 58.5 30 7.7 10.8 22.7 13.2 33.5 5.5s13.2-22.7 5.5-33.5c-21.7-30.2-57.3-50-97.5-50-12.5 0-24.6 1.9-35.9 5.5 9.8 12.7 16.5 27.9 18.9 44.6zM176 240a32 32 0 1 0 0-64 32 32 0 1 0 0 64zm192-32a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zM185.4 292.8c-2.4-2.9-5.7-4.8-9.4-4.8s-7 1.9-9.4 4.8c-6.5 7.8-12.6 16.1-18.3 24.6-9 13.4-20.3 30.2-20.3 47.4 0 28.3 21.5 51.2 48 51.2s48-22.9 48-51.2c0-17.2-11.2-34-20.3-47.4-5.7-8.5-11.9-16.7-18.3-24.6z",
            }
        }
    }
}
