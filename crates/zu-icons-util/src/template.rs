
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct {ICON_NAME};

impl IconShape for {ICON_NAME} {
    fn child_elements(&self) -> Element {
        rsx!({ICON_PATH})
    }

{OTHER_PROPS}
}
