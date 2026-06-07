use anyhow::Error;
use zu_icons_util::icon_list::{
    generate_icon_component, generate_icon_components_container, get_default_icon_crate_info,
};
use zu_icons_util::need_update_icons_page;

const CRATE_PATH: &str = "../../../crates/dioxus-icons-bs";
const OUTPUT_RS_FILE: &str = "src/icons_page.rs";
const MODULE_NAME: &str = "dioxus_icons_bs";

fn regenerate_icons() -> Result<(), Error> {
    let icons = get_default_icon_crate_info(CRATE_PATH)?;
    assert!(!icons.is_empty());
    let mut icon_components = Vec::new();
    for icon_name in &icons {
        icon_components.push(generate_icon_component(MODULE_NAME, icon_name));
    }
    generate_icon_components_container(MODULE_NAME, OUTPUT_RS_FILE, "IconsPage", &icon_components)?;

    Ok(())
}

fn main() -> Result<(), Error> {
    if need_update_icons_page() {
        regenerate_icons()?;
    }
    Ok(())
}
