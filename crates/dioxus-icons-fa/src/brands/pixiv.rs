// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PixivProps {
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

    #[props(default = Some("0 0 448 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Pixiv(props: PixivProps) -> Element {
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
        d: "M96 32C43 32 0 75 0 128L0 384c0 53 43 96 96 96l256 0c53 0 96-43 96-96l0-256c0-53-43-96-96-96L96 32zm77.7 217.3a60.1 60.1 0 1 0 120.3 0 60.1 60.1 0 1 0 -120.3 0zM119.1 387.8c-.3-.8-.5-1.7-.5-2.6l0-244.3c0-1.8 .7-3.6 2-4.9s3-2 4.9-2l16.9 0c1.2 0 2.3 .3 3.3 .8s1.9 1.3 2.5 2.3l14 21.8c19.7-15.6 44.5-25 71.6-25 63.6 0 115.2 51.6 115.2 115.2S297.5 364.5 233.8 364.5c-22 0-42.6-6.2-60.1-16.9l0 37.6c0 .9-.2 1.8-.5 2.6s-.9 1.6-1.5 2.2-1.4 1.1-2.2 1.5-1.7 .5-2.6 .5l-41.3 0c-.9 0-1.8-.2-2.6-.5s-1.6-.9-2.2-1.5-1.1-1.4-1.5-2.2z",
            }
        }
    }
}
