use anyhow::Error;
use zu_icons_util::icon_list::generate_variant_icon_pages;
use zu_icons_util::need_update_icons_page;

const CRATE_PATH: &str = "../../../crates/dioxus-icons-hero";
const MODULE_NAME: &str = "dioxus_icons_hero";

fn regenerate_icons() -> Result<(), Error> {
    generate_variant_icon_pages(CRATE_PATH, MODULE_NAME)
}

fn main() -> Result<(), Error> {
    if need_update_icons_page() {
        regenerate_icons()?;
    }
    Ok(())
}
