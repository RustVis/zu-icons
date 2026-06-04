// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

use dioxus::prelude::*;

/// This trait is used to override `IconProps`.
///
/// Implements this trait when adding a new real icon.
pub trait IconShape: Clone + PartialEq + 'static {
    /// Returns the SVG child elements (paths, circles, etc.) that define the icon shape.
    ///
    /// # Errors
    ///
    /// Returns `Err` if rendering the child elements fails.
    fn child_elements(&self) -> Element;

    /// Default title text for the SVG element.
    const TITLE: Option<&'static str> = None;

    /// Default width of the SVG element in pixels.
    const WIDTH: Option<&'static str> = None;

    /// Default height of the SVG element in pixels.
    const HEIGHT: Option<&'static str> = None;

    /// Default fill color of the SVG element.
    const FILL: Option<&'static str> = None;

    /// Default stroke color of the SVG element.
    const STROKE: Option<&'static str> = None;

    /// Default view box string (e.g., "0 0 24 24").
    const VIEW_BOX: Option<&'static str> = None;
}

/// Props for the `Icon` component.
///
/// All fields are optional except `icon`. When a field is not provided,
/// the default value from the associated `IconShape` implementation is used.
#[derive(Clone, PartialEq, Props)]
pub struct IconProps<T: IconShape> {
    /// The icon shape implementation that provides SVG child elements.
    pub icon: T,

    /// Optional title text rendered as a `<title>` element inside the SVG for accessibility.
    #[props(default = None)]
    // TODO(Shaohua): Remove and replace with aria-name/aria-label
    pub title: Option<&'static str>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # Errors
///
/// Returns `Err` if rendering the icon fails.
#[allow(non_snake_case)]
pub fn Icon<T: IconShape>(props: IconProps<T>) -> Element {
    rsx! {
        svg {
            width: T::WIDTH,
            height: T::HEIGHT,
            view_box: if let Some(view_box_text) = T::VIEW_BOX {
                view_box_text
            } else {
                "0 0 16 16"
            },
            xmlns: "http://www.w3.org/2000/svg",
            fill: if let Some(fill_text) = T::FILL {
                fill_text
            } else {
                "currentColor"
            },

            ..props.attributes,

            if let Some(title_text) = props.title {
                title {{ title_text }}
            } else if let Some(title_text) = T::TITLE {
                title {{ title_text }}
            }

            {props.icon.child_elements()}
        }
    }
}
