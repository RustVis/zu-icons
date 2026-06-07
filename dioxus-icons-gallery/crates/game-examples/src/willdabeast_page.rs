
use dioxus::prelude::*;
use dioxus_icons_game::Icon;

#[component]
pub fn WilldabeastPage() -> Element {
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
        icon: dioxus_icons_game::willdabeast::BlackBook,
    }
    span { "BlackBook" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::willdabeast::ChainLightning,
    }
    span { "ChainLightning" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::willdabeast::ChainMail,
    }
    span { "ChainMail" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::willdabeast::GoldBar,
    }
    span { "GoldBar" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::willdabeast::OrbWand,
    }
    span { "OrbWand" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::willdabeast::RoundShield,
    }
    span { "RoundShield" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::willdabeast::WhiteBook,
    }
    span { "WhiteBook" }
}

        }
  )
}
