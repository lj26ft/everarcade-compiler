use crate::layout::PanelKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioWindow {
    pub title: String,
    pub active_panel: PanelKind,
    pub native_desktop: bool,
    pub browser_dependency: bool,
}

impl Default for StudioWindow {
    fn default() -> Self {
        Self {
            title: "EverArcade Studio".to_owned(),
            active_panel: PanelKind::Viewport,
            native_desktop: true,
            browser_dependency: false,
        }
    }
}
