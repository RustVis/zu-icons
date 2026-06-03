// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct {ICON_NAME}Props {
    #[props(default = None)]
    pub title: Option<&'static str>,

    #[props(default = None)]
    pub class: Option<&'static str>,

    #[props(default = None)]
    pub style: Option<&'static str>,

    #[props(default = {WIDTH})]
    pub width: Option<&'static str>,

    #[props(default = {HEIGHT})]
    pub height: Option<&'static str>,

    #[props(default = {FILLE})]
    pub fill: Option<&'static str>,

    #[props(default = {STROKE})]
    pub stroke: Option<&'static str>,

    #[props(default = {VIEW_BOX})]
    pub view_box: Option<&'static str>,

    #[props(default = {XMLNS})]
    pub xmlns: Option<&'static str>,
}

pub fn {ICON_NAME}(props: {ICON_NAME}Props) -> Element {
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

            {ICON_PATH}
        }
    }
}
