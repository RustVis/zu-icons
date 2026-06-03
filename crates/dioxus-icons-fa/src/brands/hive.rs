// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HiveProps {
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

pub fn Hive(props: HiveProps) -> Element {
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
        d: "M260.4 254.9L131.5 33.1c-.2-.3-.5-.6-.8-.8s-.7-.3-1.1-.3-.8 .1-1.1 .3-.6 .5-.8 .8L.3 254.9c-.2 .3-.3 .7-.3 1.1s.1 .8 .3 1.1L129.1 478.9c.2 .3 .5 .6 .8 .8s.7 .3 1.1 .3 .8-.1 1.1-.3 .6-.5 .8-.8L260.4 257.1c.2-.3 .3-.7 .3-1.1s-.1-.8-.3-1.1zm39.1-25.7c.2 .3 .5 .6 .8 .8s.7 .3 1.1 .3l66.5 0c.4 0 .8-.1 1.1-.3s.6-.5 .8-.8 .3-.7 .3-1.1-.1-.8-.3-1.1L259.1 33.1c-.2-.3-.5-.6-.8-.8s-.7-.3-1.1-.3l-66.5 0c-.4 0-.8 .1-1.1 .3s-.6 .5-.8 .8-.3 .7-.3 1.1 .1 .8 .3 1.1L299.4 229.2zm212.3 25.7L384.9 33.1c-.2-.3-.5-.6-.8-.8s-.7-.3-1.1-.3l-66.6 0c-.4 0-.8 .1-1.1 .3s-.6 .5-.8 .8-.3 .7-.3 1.1 .1 .8 .3 1.1L440.7 256 314.5 476.7c-.2 .3-.3 .7-.3 1.1s.1 .8 .3 1.1 .5 .6 .8 .8 .7 .3 1.1 .3l66.6 0c.4 0 .8-.1 1.1-.3s.6-.5 .8-.8L511.7 257.1c.2-.3 .3-.7 .3-1.1s-.1-.8-.3-1.1zM366 284.9l-66.5 0c-.4 0-.8 .1-1.1 .3s-.6 .5-.8 .8L188.8 476.7c-.2 .3-.3 .7-.3 1.1s.1 .8 .3 1.1 .5 .6 .8 .8 .7 .3 1.1 .3l66.5 0c.4 0 .8-.1 1.1-.3s.6-.5 .8-.8L367.9 288.3c.2-.3 .3-.7 .3-1.1s-.1-.8-.3-1.1-.5-.6-.8-.8-.7-.3-1.1-.3z",
            }
        }
    }
}
