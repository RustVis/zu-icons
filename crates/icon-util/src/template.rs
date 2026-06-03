
#[cfg(feature = "{FEATURE_NAME}")]
#[derive(Copy, Clone, PartialEq)]
pub struct {ICON_NAME} {}

#[cfg(feature = "{FEATURE_NAME}")]
impl IconShape for {ICON_NAME} {
    fn child_elements(&self) -> Element {
        rsx!({ICON_PATH})
    }

{OTHER_PROPS}
}
