
use dioxus::prelude::*;
use dioxus_icons_game::Icon;

#[component]
pub fn QuotingPage() -> Element {
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
        icon: dioxus_icons_game::quoting::ArtilleryShell,
    }
    span { "ArtilleryShell" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::Biplane,
    }
    span { "Biplane" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::Bunker,
    }
    span { "Bunker" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::BunkerAssault,
    }
    span { "BunkerAssault" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::CardPickup,
    }
    span { "CardPickup" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::CardPlay,
    }
    span { "CardPlay" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::DamagedHouse,
    }
    span { "DamagedHouse" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::FieldGun,
    }
    span { "FieldGun" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::LostLimb,
    }
    span { "LostLimb" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::MilitaryAmbulance,
    }
    span { "MilitaryAmbulance" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::Poppy,
    }
    span { "Poppy" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::quoting::TrenchAssault,
    }
    span { "TrenchAssault" }
}

        }
  )
}
