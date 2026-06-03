// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CodePullRequestProps {
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

pub fn CodePullRequest(props: CodePullRequestProps) -> Element {
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
        d: "M328 24c0-9.7-5.8-18.5-14.8-22.2S293.9 .2 287 7L231 63c-9.4 9.4-9.4 24.6 0 33.9l56 56c6.9 6.9 17.2 8.9 26.2 5.2S328 145.7 328 136l0-24 24 0c17.7 0 32 14.3 32 32l0 214.7c-28.3 12.3-48 40.5-48 73.3 0 44.2 35.8 80 80 80s80-35.8 80-80c0-32.8-19.7-61-48-73.3L448 144c0-53-43-96-96-96l-24 0 0-24zM72 80a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zm56 73.3c28.3-12.3 48-40.5 48-73.3 0-44.2-35.8-80-80-80S16 35.8 16 80c0 32.8 19.7 61 48 73.3l0 205.3c-28.3 12.3-48 40.5-48 73.3 0 44.2 35.8 80 80 80s80-35.8 80-80c0-32.8-19.7-61-48-73.3l0-205.3zM72 432a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zm344-24a24 24 0 1 1 0 48 24 24 0 1 1 0-48z",
            }
        }
    }
}
