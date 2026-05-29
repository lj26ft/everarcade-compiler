use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PanelKind {
    Hierarchy,
    Inspector,
    Viewport,
    Assets,
    Replay,
    Simulation,
    Diagnostics,
    Deployment,
    Console,
}

impl PanelKind {
    pub const ALL: [PanelKind; 9] = [
        PanelKind::Hierarchy,
        PanelKind::Inspector,
        PanelKind::Viewport,
        PanelKind::Assets,
        PanelKind::Replay,
        PanelKind::Simulation,
        PanelKind::Diagnostics,
        PanelKind::Deployment,
        PanelKind::Console,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            PanelKind::Hierarchy => "Hierarchy",
            PanelKind::Inspector => "Inspector",
            PanelKind::Viewport => "Viewport",
            PanelKind::Assets => "Assets",
            PanelKind::Replay => "Replay",
            PanelKind::Simulation => "Simulation",
            PanelKind::Diagnostics => "Diagnostics",
            PanelKind::Deployment => "Deployment",
            PanelKind::Console => "Console",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DockPanel {
    pub kind: PanelKind,
    pub dock: &'static str,
    pub visible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceLayout {
    pub layout_id: String,
    pub panels: Vec<DockPanel>,
    pub selected_panel: PanelKind,
    pub deterministic_state_hash: String,
}

impl WorkspaceLayout {
    pub fn default_creator_layout() -> Self {
        Self::new(
            "creator-default",
            vec![
                DockPanel {
                    kind: PanelKind::Hierarchy,
                    dock: "left",
                    visible: true,
                },
                DockPanel {
                    kind: PanelKind::Inspector,
                    dock: "right",
                    visible: true,
                },
                DockPanel {
                    kind: PanelKind::Viewport,
                    dock: "center",
                    visible: true,
                },
                DockPanel {
                    kind: PanelKind::Assets,
                    dock: "bottom",
                    visible: true,
                },
                DockPanel {
                    kind: PanelKind::Replay,
                    dock: "bottom",
                    visible: true,
                },
                DockPanel {
                    kind: PanelKind::Simulation,
                    dock: "right",
                    visible: true,
                },
                DockPanel {
                    kind: PanelKind::Diagnostics,
                    dock: "bottom",
                    visible: true,
                },
                DockPanel {
                    kind: PanelKind::Deployment,
                    dock: "workspace",
                    visible: true,
                },
                DockPanel {
                    kind: PanelKind::Console,
                    dock: "bottom",
                    visible: true,
                },
            ],
        )
    }

    pub fn new(layout_id: &str, mut panels: Vec<DockPanel>) -> Self {
        panels.sort_by(|a, b| a.kind.cmp(&b.kind));
        let hash = layout_hash(layout_id, &panels);
        Self {
            layout_id: layout_id.to_owned(),
            panels,
            selected_panel: PanelKind::Viewport,
            deterministic_state_hash: hash,
        }
    }

    pub fn serialize(&self) -> String {
        let rows = self
            .panels
            .iter()
            .map(|panel| format!("{}:{}:{}", panel.kind.label(), panel.dock, panel.visible))
            .collect::<Vec<_>>()
            .join(";");
        format!(
            "{}|{}|{}",
            self.layout_id,
            self.selected_panel.label(),
            rows
        )
    }

    pub fn restore(serialized: &str) -> Self {
        let mut parts = serialized.split('|');
        let layout_id = parts.next().unwrap_or("creator-default");
        let selected = panel_from_label(parts.next().unwrap_or("Viewport"));
        let rows = parts.next().unwrap_or_default();
        let mut panels = rows
            .split(';')
            .filter_map(|row| {
                let mut cols = row.split(':');
                let kind = panel_from_label(cols.next()?);
                let dock = match cols.next().unwrap_or("workspace") {
                    "left" => "left",
                    "right" => "right",
                    "center" => "center",
                    "bottom" => "bottom",
                    _ => "workspace",
                };
                let visible = cols.next().unwrap_or("true") == "true";
                Some(DockPanel {
                    kind,
                    dock,
                    visible,
                })
            })
            .collect::<Vec<_>>();
        if panels.is_empty() {
            panels = Self::default_creator_layout().panels;
        }
        let mut restored = Self::new(layout_id, panels);
        restored.selected_panel = selected;
        restored
    }

    pub fn has_all_required_panels(&self) -> bool {
        PanelKind::ALL.iter().all(|required| {
            self.panels
                .iter()
                .any(|panel| &panel.kind == required && panel.visible)
        })
    }
}

fn panel_from_label(label: &str) -> PanelKind {
    match label {
        "Hierarchy" => PanelKind::Hierarchy,
        "Inspector" => PanelKind::Inspector,
        "Assets" => PanelKind::Assets,
        "Replay" => PanelKind::Replay,
        "Simulation" => PanelKind::Simulation,
        "Diagnostics" => PanelKind::Diagnostics,
        "Deployment" => PanelKind::Deployment,
        "Console" => PanelKind::Console,
        _ => PanelKind::Viewport,
    }
}

fn layout_hash(layout_id: &str, panels: &[DockPanel]) -> String {
    let mut parts = vec!["workspace-layout".to_owned(), layout_id.to_owned()];
    parts.extend(
        panels
            .iter()
            .map(|panel| format!("{}:{}:{}", panel.kind.label(), panel.dock, panel.visible)),
    );
    stable_hash(&parts.iter().map(String::as_str).collect::<Vec<_>>())
}
