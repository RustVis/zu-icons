
use dioxus::prelude::*;
use dioxus_icons_game::Icon;

#[component]
pub fn Generalace135Page() -> Element {
  rsx!(
    div {
        display: "flex",
        flex_direction: "row",
        gap: "24px",
        flex_wrap: "wrap",
            
div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::generalace135::FingersCrossed,
    }
    span { "FingersCrossed" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::generalace135::ShepherdsCrook,
    }
    span { "ShepherdsCrook" }
}

        }
  )
}
