use anyhow::Error;
use inflections::Inflect;
use zu_icons_util::icon_list::{
    generate_icon_component, generate_icon_components_container, get_variant_icon_crate_info,
};
use zu_icons_util::need_update_icons_page;

const CRATE_PATH: &str = "../../../crates/dioxus-icons-ant";
const MODULE_NAME: &str = "dioxus_icons_ant";

fn regenerate_icons() -> Result<(), Error> {
    let all_icons = get_variant_icon_crate_info(CRATE_PATH)?;
    assert!(!all_icons.is_empty());
    for (feature_name, icons) in &all_icons {
        let mut icon_components = Vec::new();
        let module_feature_name = format!("{MODULE_NAME}::{feature_name}");
        for icon_name in icons {
            icon_components.push(generate_icon_component(&module_feature_name, icon_name));
        }
        generate_icon_components_container(
            MODULE_NAME,
            &format!("src/{feature_name}_page.rs"),
            &format!("{}Page", feature_name.to_pascal_case()),
            &icon_components,
        )?;
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    if need_update_icons_page() {
        regenerate_icons()?;
    }
    Ok(())
}
