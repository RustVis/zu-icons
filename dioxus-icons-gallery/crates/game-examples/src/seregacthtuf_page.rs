
use dioxus::prelude::*;
use dioxus_icons_game::Icon;

#[component]
pub fn SeregacthtufPage() -> Element {
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
        icon: dioxus_icons_game::seregacthtuf::AcidShield,
    }
    span { "AcidShield" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::seregacthtuf::ArmorBlueprint,
    }
    span { "ArmorBlueprint" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::seregacthtuf::PouchWithBeads,
    }
    span { "PouchWithBeads" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::seregacthtuf::SakeBottle,
    }
    span { "SakeBottle" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::seregacthtuf::ThoughtBubble,
    }
    span { "ThoughtBubble" }
}

        }
  )
}
