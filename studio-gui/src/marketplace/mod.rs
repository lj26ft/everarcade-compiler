#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketplaceEntry {
    pub package_id: String,
    pub name: String,
    pub category: String,
    pub dependencies: Vec<String>,
    pub validation_status: String,
    pub installed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketplaceBrowser {
    pub entries: Vec<MarketplaceEntry>,
}

impl MarketplaceBrowser {
    pub fn sample() -> Self {
        Self {
            entries: ["Combat", "Inventory", "Quest", "Dialogue", "Economy"]
                .into_iter()
                .map(|name| MarketplaceEntry {
                    package_id: format!("arena-vanguard-{}", name.to_ascii_lowercase()),
                    name: name.to_owned(),
                    category: name.to_owned(),
                    dependencies: Vec::new(),
                    validation_status: "certified".to_owned(),
                    installed: false,
                })
                .collect(),
        }
    }

    pub fn browse_packages(&self) -> &[MarketplaceEntry] {
        &self.entries
    }

    pub fn search_packages(&self, query: &str) -> Vec<MarketplaceEntry> {
        let query = query.to_ascii_lowercase();
        self.entries
            .iter()
            .filter(|entry| entry.name.to_ascii_lowercase().contains(&query))
            .cloned()
            .collect()
    }

    pub fn install_package(&mut self, package_id: &str) -> bool {
        self.set_installed(package_id, true)
    }

    pub fn update_package(&mut self, package_id: &str) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.package_id == package_id && entry.validation_status == "certified")
    }

    pub fn remove_package(&mut self, package_id: &str) -> bool {
        self.set_installed(package_id, false)
    }

    pub fn view_dependencies(&self, package_id: &str) -> Option<Vec<String>> {
        self.entries
            .iter()
            .find(|entry| entry.package_id == package_id)
            .map(|entry| entry.dependencies.clone())
    }

    pub fn view_validation_status(&self, package_id: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|entry| entry.package_id == package_id)
            .map(|entry| entry.validation_status.clone())
    }

    fn set_installed(&mut self, package_id: &str, installed: bool) -> bool {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.package_id == package_id)
        {
            entry.installed = installed;
            true
        } else {
            false
        }
    }
}
