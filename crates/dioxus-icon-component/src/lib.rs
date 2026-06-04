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
    const WIDTH: Option<u32> = None;
    /// Default height of the SVG element in pixels.
    const HEIGHT: Option<u32> = None;
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
#[derive(Clone, PartialEq, Eq, Props)]
pub struct IconProps<T: IconShape> {
    /// The icon shape implementation that provides SVG child elements.
    pub icon: T,

    /// Optional title text rendered as a `<title>` element inside the SVG for accessibility.
    #[props(default = None)]
    pub title: Option<&'static str>,

    /// Optional CSS class name(s) applied to the `<svg>` element.
    #[props(default = None)]
    pub class: Option<&'static str>,

    /// Optional inline CSS styles applied to the `<svg>` element.
    #[props(default = None)]
    pub style: Option<&'static str>,

    /// Optional width of the SVG element in pixels.
    /// Falls back to `T::WIDTH` if not set.
    #[props(default = None)]
    pub width: Option<u32>,

    /// Optional height of the SVG element in pixels.
    /// Falls back to `T::HEIGHT` if not set.
    #[props(default = None)]
    pub height: Option<u32>,

    /// Optional fill color for the SVG element.
    /// Falls back to `T::FILL`, then to `"currentColor"` if not set.
    #[props(default = None)]
    pub fill: Option<&'static str>,

    /// Optional stroke color for the SVG element.
    /// Falls back to `T::STROKE` if not set.
    #[props(default = None)]
    pub stroke: Option<&'static str>,

    /// Optional view box attribute (e.g., `"0 0 24 24"`).
    /// Falls back to `T::VIEW_BOX`, then to `"0 0 16 16"` if not set.
    #[props(default = None)]
    pub view_box: Option<&'static str>,

    /// Optional XML namespace override.
    /// Defaults to `"http://www.w3.org/2000/svg"` if not set.
    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

/// # Errors
///
/// Returns `Err` if rendering the icon fails.
#[component]
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
