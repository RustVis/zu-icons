use anyhow::Error;
use zu_icons_util::icon_list::generate_default_icon_page;
use zu_icons_util::need_update_icons_page;

const CRATE_PATH: &str = "../../../crates/dioxus-icons-circum";
const OUTPUT_RS_FILE: &str = "src/icons_page.rs";
const MODULE_NAME: &str = "dioxus_icons_circum";

fn regenerate_icons() -> Result<(), Error> {
    generate_default_icon_page(CRATE_PATH, OUTPUT_RS_FILE, MODULE_NAME)
}

fn main() -> Result<(), Error> {
    if need_update_icons_page() {
        regenerate_icons()?;
    }
    Ok(())
}
