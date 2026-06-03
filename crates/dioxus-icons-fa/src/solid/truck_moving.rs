// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TruckMovingProps {
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

pub fn TruckMoving(props: TruckMovingProps) -> Element {
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
        d: "M64 32C28.7 32 0 60.7 0 96L0 424c0 48.6 39.4 88 88 88 25.2 0 48-10.6 64-27.6 16 17 38.8 27.6 64 27.6 40.3 0 74.2-27.1 84.7-64l134.6 0c10.4 36.9 44.4 64 84.7 64 43 0 78.9-30.9 86.5-71.7 20-10.8 33.5-32 33.5-56.3l0-146.7c0-17-6.7-33.3-18.7-45.3L576 146.7c-12-12-28.3-18.7-45.3-18.7l-50.7 0 0-32c0-35.3-28.7-64-64-64L64 32zM480 192l50.7 0 45.3 45.3 0 50.7-96 0 0-96zM88 384a40 40 0 1 1 0 80 40 40 0 1 1 0-80zm392 40a40 40 0 1 1 80 0 40 40 0 1 1 -80 0zM216 384a40 40 0 1 1 0 80 40 40 0 1 1 0-80z",
            }
        }
    }
}
