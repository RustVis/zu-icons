// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use dioxus::prelude::*;

/// This trait is used to override `IconProps`.
///
/// Implements this trait when adding a new real icon.
pub trait IconShape: Clone + PartialEq + 'static {
    fn child_elements(&self) -> Element;

    const TITLE: Option<&'static str> = None;
    const WIDTH: Option<u32> = None;
    const HEIGHT: Option<u32> = None;
    const FILL: Option<&'static str> = None;
    const STROKE: Option<&'static str> = None;
    const VIEW_BOX: Option<&'static str> = None;
}

#[derive(Clone, PartialEq, Props)]
pub struct IconProps<T: IconShape> {
    pub icon: T,

    #[props(default = None)]
    pub title: Option<&'static str>,

    #[props(default = None)]
    pub class: Option<&'static str>,

    #[props(default = None)]
    pub style: Option<&'static str>,

    #[props(default = None)]
    pub width: Option<u32>,

    #[props(default = None)]
    pub height: Option<u32>,

    #[props(default = None)]
    pub fill: Option<&'static str>,

    #[props(default = None)]
    pub stroke: Option<&'static str>,

    #[props(default = None)]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

#[allow(non_snake_case)]
pub fn Icon<T: IconShape>(props: IconProps<T>) -> Element {
    rsx! {
        svg {
            class: props.class,
            style: props.style,
            width: if props.width.is_some(){ props.width } else { T::WIDTH },
            height: if props.height.is_some() { props.height } else { T::HEIGHT },
            view_box: if props.view_box.is_some() {
                props.view_box
            } else if T::VIEW_BOX.is_some() {
               T::VIEW_BOX
            } else {
                "0 0 16 16"
            },
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg"),
            fill: if props.fill.is_some() {
                props.fill
            } else if T::FILL.is_some() {
                T::FILL
            } else {
                "currentColor"
            },
            stroke: props.stroke,

            if let Some(title_text) = props.title {
                title {{ title_text }}
            } else if let Some(title_text) = T::TITLE {
                title {{ title_text }}
            }

            {props.icon.child_elements()}
        }
    }
}
