// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TruckLoadingProps {
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

pub fn TruckLoading(props: TruckLoadingProps) -> Element {
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
        d: "M400 32c-44.2 0-80 35.8-80 80l0 248.2-297.2 89.2C5.9 454.4-3.7 472.3 1.4 489.2s22.9 26.5 39.8 21.5l315.5-94.6 43.7 0c-.2 2.6-.4 5.3-.4 8 0 48.6 39.4 88 88 88s88-39.4 88-88l0-392-176 0zM528 424c0 22.1-17.9 40-40 40s-40-17.9-40-40 17.9-40 40-40c22.1 0 39.9 17.9 40 39.9l0 .1zM51.9 149.5C18.1 159.8-.9 195.6 9.4 229.4l28.1 91.8C47.8 355 83.6 374 117.4 363.7l91.8-28.1c33.8-10.3 52.8-46.1 42.5-79.9l-28.1-91.8c-10.3-33.8-46.1-52.8-79.9-42.5L51.9 149.5z",
            }
        }
    }
}
