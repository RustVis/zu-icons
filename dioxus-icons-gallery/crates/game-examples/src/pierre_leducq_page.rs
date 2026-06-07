
use dioxus::prelude::*;
use dioxus_icons_game::Icon;

#[component]
pub fn PierreLeducqPage() -> Element {
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
        icon: dioxus_icons_game::pierre_leducq::AntiAircraftGun,
    }
    span { "AntiAircraftGun" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::pierre_leducq::ShoonerSailboat,
    }
    span { "ShoonerSailboat" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::pierre_leducq::SmallFishingSailboat,
    }
    span { "SmallFishingSailboat" }
}

        }
  )
}
