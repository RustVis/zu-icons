// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MailBulkProps {
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

pub fn MailBulk(props: MailBulkProps) -> Element {
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
        d: "M112 0C85.5 0 64 21.5 64 48l0 160 80 0 0-32c0-53 43-96 96-96l208 0 0-32c0-26.5-21.5-48-48-48L112 0zM240 128c-26.5 0-48 21.5-48 48l0 32 80 0c53 0 96 43 96 96l0 112 160 0c26.5 0 48-21.5 48-48l0-192c0-26.5-21.5-48-48-48l-288 0zm200 64l48 0c13.3 0 24 10.7 24 24l0 48c0 13.3-10.7 24-24 24l-48 0c-13.3 0-24-10.7-24-24l0-48c0-13.3 10.7-24 24-24zM48 256c-26.5 0-48 21.5-48 48l0 10.4 156.6 86.2c1.1 .6 2.2 .9 3.4 .9s2.4-.3 3.4-.9L320 314.4 320 304c0-26.5-21.5-48-48-48L48 256zM320 369.2L186.6 442.6c-8.1 4.5-17.3 6.8-26.6 6.8s-18.4-2.4-26.6-6.8L0 369.2 0 464c0 26.5 21.5 48 48 48l224 0c26.5 0 48-21.5 48-48l0-94.8z",
            }
        }
    }
}
