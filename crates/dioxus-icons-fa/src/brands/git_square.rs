// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GitSquareProps {
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

pub fn GitSquare(props: GitSquareProps) -> Element {
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
        d: "M120.8 335.5c-5.9-.4-12.6-.8-20.2-1.3-3.3 4.1-6.6 8.4-6.6 13.5 0 18.5 65.5 18.5 65.5-1.5 0-8.3-7.4-8.7-38.8-10.7l.1 0zm7.8-117.9c-32.3 0-33.7 44.5-.7 44.5 32.5 0 31.7-44.5 .7-44.5zM384 32L64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64zM243.9 172.2c-14.5 0-22.9-8.4-22.9-22.9s8.4-22.3 22.9-22.3c14.7 0 23.1 7.8 23.1 22.3s-8.4 22.9-23.1 22.9zM149.6 195l49.5 0 0 21.6-23.4 1.8c4.6 5.8 9.4 14 9.4 25.7 0 48.7-57.2 47.2-74.2 42.4l-8.4 13.4c5 .3 9.8 .6 14.3 .8 56.3 3.2 80.5 4.6 80.5 38.5 0 29.2-25.7 45.7-69.9 45.7-46 0-63.5-11.6-63.5-31.7 0-11.4 5.1-17.5 14-25.9-8.4-3.5-11.2-9.9-11.2-16.8 0-9.6 7.4-16.3 23-30.6l.2-.2c-12.4-6.1-21.8-19.3-21.8-38.1 0-51.6 56.6-53.3 81.6-46.8l-.1 .2zM270.5 303.1l13 1.8 0 20.1-72.4 0 0-20.1c2.7-.4 5-.7 6.9-.9 9.9-1.2 10.1-1.3 10.1-6l0-74.7c0-4.4-.9-4.7-10.1-7.8-1.9-.7-4.2-1.4-6.9-2.4l2.8-20.6 52.6 0 0 105.5c0 4.1 .2 4.6 4.1 5.1l-.1 0zm106.6-10.4L384 315c-10.9 5.4-26.9 10.2-41.4 10.2-30.2 0-41.7-12.2-41.7-40.9l0-66.6c0-.8 0-1.4-.2-1.8-.8-1.2-4.2-.7-19.6-.7l0-22.6c22.3-2.5 31.2-13.7 34-41.4l24.2 0c0 33.3-.6 38 .7 38.6 .3 .1 .7 0 1.3 0l35.8 0 0 25.4-37.8 0 0 61.6c-.2 6.3-.9 30.4 37.9 15.9l-.1 0z",
            }
        }
    }
}
