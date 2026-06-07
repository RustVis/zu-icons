
use dioxus::prelude::*;
use dioxus_icons_game::Icon;

#[component]
pub fn JohnRedmanPage() -> Element {
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
        icon: dioxus_icons_game::john_redman::Paper,
    }
    span { "Paper" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::john_redman::Rock,
    }
    span { "Rock" }
}

div {
    display: "flex",
    flex_direction: "column",
    align_items: "center",
    Icon {
        width: "32",
        height: "32",
        icon: dioxus_icons_game::john_redman::Scissors,
    }
    span { "Scissors" }
}

        }
  )
}
