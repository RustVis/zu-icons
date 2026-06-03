// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SolanaProps {
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

pub fn Solana(props: SolanaProps) -> Element {
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
        d: "M510.5 385.2l-84.9 88.7c-1.8 1.9-4.1 3.5-6.6 4.5s-5.2 1.6-7.9 1.6L8.9 480c-1.9 0-3.8-.5-5.4-1.6s-2.9-2.5-3.6-4.2-1-3.6-.7-5.5 1.2-3.6 2.5-4.9l84.9-88.7c1.8-1.9 4.1-3.5 6.5-4.5s5.1-1.6 7.8-1.6l402.4 0c1.9 0 3.8 .5 5.4 1.6s2.9 2.5 3.6 4.2 1 3.6 .7 5.5-1.2 3.6-2.5 4.9zM425.7 206.6c-1.8-1.9-4.1-3.5-6.6-4.5s-5.2-1.6-7.9-1.6l-402.4 0c-1.9 0-3.8 .5-5.4 1.6s-2.9 2.5-3.6 4.2-1 3.6-.7 5.5 1.2 3.6 2.5 4.9l84.9 88.7c1.8 1.9 4.1 3.5 6.5 4.5s5.1 1.6 7.8 1.6l402.4 0c1.9 0 3.8-.5 5.4-1.6s2.9-2.5 3.6-4.2 1-3.6 .7-5.5-1.2-3.6-2.5-4.9l-84.9-88.7zM8.9 142.9l402.4 0c2.7 0 5.4-.5 7.9-1.6s4.7-2.6 6.6-4.5l84.9-88.7c1.3-1.4 2.2-3.1 2.5-4.9s.1-3.7-.7-5.5-2-3.2-3.6-4.2-3.5-1.6-5.4-1.6L101 32c-2.7 0-5.4 .5-7.8 1.6s-4.7 2.6-6.5 4.5L1.7 126.8c-1.3 1.4-2.2 3.1-2.5 4.9s-.1 3.7 .7 5.5 2 3.2 3.6 4.2 3.5 1.6 5.4 1.6z",
            }
        }
    }
}
