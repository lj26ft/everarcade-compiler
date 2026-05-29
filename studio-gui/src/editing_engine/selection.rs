use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectionMode {
    Single,
    Multi,
    Group,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionSet {
    pub ids: Vec<String>,
    pub groups: Vec<String>,
    pub mode: SelectionMode,
    pub selection_hash: String,
}

impl SelectionSet {
    pub fn empty() -> Self {
        Self {
            ids: Vec::new(),
            groups: Vec::new(),
            mode: SelectionMode::Single,
            selection_hash: stable_hash(&["selection", "empty"]),
        }
    }

    pub fn select<I, S>(&mut self, ids: I, mode: SelectionMode)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut ids: Vec<String> = ids.into_iter().map(Into::into).collect();
        ids.sort();
        ids.dedup();
        self.ids = ids;
        self.mode = mode;
        self.rebuild_hash();
    }

    pub fn group(&mut self, group_id: &str) {
        if !self.groups.iter().any(|group| group == group_id) {
            self.groups.push(group_id.to_owned());
            self.groups.sort();
        }
        self.mode = SelectionMode::Group;
        self.rebuild_hash();
    }

    fn rebuild_hash(&mut self) {
        let mode = match self.mode {
            SelectionMode::Single => "single",
            SelectionMode::Multi => "multi",
            SelectionMode::Group => "group",
        };
        let ids = self.ids.join(",");
        let groups = self.groups.join(",");
        self.selection_hash = stable_hash(&["selection", mode, &ids, &groups]);
    }
}
