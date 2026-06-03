// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FoursquareProps {
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

    #[props(default = Some("0 0 384 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Foursquare(props: FoursquareProps) -> Element {
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
        d: "M331.1 3L57.9 3C20.4 3 8 31.3 8 49.1L8 482.9c0 20.3 12.1 27.7 18.2 30.1 6.2 2.5 22.8 4.6 32.9-7.1 128.9-149.4 131.1-151.9 131.1-151.9 3.1-3.4 3.4-3.1 6.8-3.1l83.4 0c35.1 0 40.6-25.2 44.3-39.7l48.6-243C381.8 25.8 371.1 3 331.1 3zM314.8 76.8l-11.4 59.7c-1.2 6.5-9.5 13.2-16.9 13.2l-106.4 0c-12 0-20.6 8.3-20.6 20.3l0 13c0 12 8.6 20.6 20.6 20.6l90.4 0c8.3 0 16.6 9.2 14.8 18.2-1.8 8.9-10.5 53.8-11.4 58.8-.9 4.9-6.8 13.5-16.9 13.5l-73.5 0c-13.5 0-17.2 1.8-26.5 12.6 0 0-8.9 11.4-89.5 108.3-.9 .9-1.8 .6-1.8-.3l0-338.8c0-7.7 6.8-16.6 16.6-16.6l219 0c8.2 0 15.6 7.7 13.5 17.5z",
            }
        }
    }
}
