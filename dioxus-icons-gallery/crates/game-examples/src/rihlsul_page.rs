
use dioxus::prelude::*;
use dioxus_icons_game::Icon;

#[component]
pub fn RihlsulPage() -> Element {
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
        icon: dioxus_icons_game::rihlsul::ChocolateBar,
    }
    span { "ChocolateBar" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::rihlsul::MilkCarton,
    }
    span { "MilkCarton" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::rihlsul::Peanut,
    }
    span { "Peanut" }
}

        }
  )
}
