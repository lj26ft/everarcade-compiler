#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComposerNode {
    pub package_id: String,
    pub record_types: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RustrigComposer {
    pub nodes: Vec<ComposerNode>,
    pub connections: Vec<(String, String)>,
}

impl RustrigComposer {
    pub fn drag_package(&mut self, package_id: &str) {
        self.nodes.push(ComposerNode {
            package_id: package_id.to_owned(),
            record_types: vec![format!("{}Record", package_id.replace('-', "_"))],
            dependencies: Vec::new(),
        });
    }

    pub fn connect_rustrigs(&mut self, from: &str, to: &str) -> bool {
        let has_from = self.nodes.iter().any(|node| node.package_id == from);
        let has_to = self.nodes.iter().any(|node| node.package_id == to);
        if has_from && has_to {
            self.connections.push((from.to_owned(), to.to_owned()));
            true
        } else {
            false
        }
    }

    pub fn visual_composition(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| node.package_id.clone())
            .collect()
    }

    pub fn record_inspection(&self, package_id: &str) -> Option<Vec<String>> {
        self.nodes
            .iter()
            .find(|node| node.package_id == package_id)
            .map(|node| node.record_types.clone())
    }

    pub fn dependency_inspection(&self, package_id: &str) -> Option<Vec<String>> {
        self.nodes
            .iter()
            .find(|node| node.package_id == package_id)
            .map(|node| node.dependencies.clone())
    }
}
