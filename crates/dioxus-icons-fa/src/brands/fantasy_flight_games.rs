// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FantasyFlightGamesProps {
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

pub fn FantasyFlightGames(props: FantasyFlightGamesProps) -> Element {
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
        d: "M256 32.9L32.9 256 256 479.1 479.1 256 256 32.9zM88.3 255.8c2-2 11.9-12.3 96.5-97.5 41.4-41.8 86.2-43.8 119.8-18.7 24.6 18.4 62.1 58.9 62.1 59 .7 .7 1.1 2.9 .6 3.4-11.3 11.8-22.7 23.5-33.5 34.7-34.2-32.3-40.5-38.2-48.5-44-17.8-12.7-41.4-10.1-57 5.1-2.2 2.1-1.8 3.4 .1 5.4 2.9 2.9 28.1 28.3 35.1 35.8-11.9 11.6-23.7 23-35.7 34.7-12-12.5-24.5-25.5-36.5-38.1-21.4 21.1-41.7 41.1-61.8 61L88.3 255.8zM323.2 357.4c-35.5 35.4-78.1 38.1-107 20.5-22.1-13.5-39.4-32.1-72.9-66.8 12-12.4 23.8-24.4 35.4-36.3 33 31.9 37.1 36 44.7 42.1 18.5 14.7 42.5 13.7 59.3-1.8 3.7-3.4 3.7-3.6 .1-7.2-10.6-10.7-21.2-21.4-31.8-32.2-1.3-1.3-3-2.5-.8-4.7 10.8-10.7 21.5-21.5 32.2-32.3 .3-.3 .6-.4 1.9-1.1 12.4 12.9 24.9 25.9 37.2 38.8 21-20.7 41.2-40.7 61.3-60.4 13.7 13.4 27.1 26.6 40.9 40-20.2 20.9-81.7 82.7-100.5 101.5zM256 0L0 256 256 512 512 256 256 0zM16 256L256 16 496 256 256 496 16 256z",
            }
        }
    }
}
