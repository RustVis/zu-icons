// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PersonMilitaryRifleProps {
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

pub fn PersonMilitaryRifle(props: PersonMilitaryRifleProps) -> Element {
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
        d: "M128 39c0-13 10-23.8 22.9-24.9L302.7 1.4C312 .7 320 8 320 17.4L320 48c0 8.8-7.2 16-16 16L153 64c-13.8 0-25-11.2-25-25zm17.6 57l156.8 0c1 5.2 1.6 10.5 1.6 16 0 44.2-35.8 80-80 80s-80-35.8-80-80c0-5.5 .6-10.8 1.6-16zm228 364.3L320 369.7 320 480c0 1.3-.1 2.5-.2 3.8L145.5 234.9c16.6-7.1 34.6-10.9 53.3-10.9l50.4 0c15.9 0 31.3 2.8 45.8 7.9L389.9 67.7c-7.7-4.4-10.3-14.2-5.9-21.9s14.2-10.3 21.9-5.9l27.7 16c7.7 4.4 10.3 14.2 5.9 21.9l-55.5 96.1 1.6 .9c15.3 8.8 20.6 28.4 11.7 43.7L360.7 282c2 2.8 3.9 5.8 5.7 8.8l76.1 128.8c11.2 19 4.9 43.5-14.1 54.8s-43.5 4.9-54.8-14.1zM288 512l-128 0c-17.7 0-32-14.3-32-32l0-110.3-53.6 90.6c-11.2 19-35.8 25.3-54.8 14.1S-5.7 438.7 5.6 419.7L81.7 290.8c9.4-15.8 21.7-29.3 36-40L299.1 510c-3.5 1.3-7.2 2-11.1 2zM264 320a24 24 0 1 0 0-48 24 24 0 1 0 0 48z",
            }
        }
    }
}
