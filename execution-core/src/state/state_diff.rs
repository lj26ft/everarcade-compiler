#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateInsert {
    pub key: String,
    pub value: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateUpdate {
    pub key: String,
    pub value: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateRemoval {
    pub key: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StateDiff {
    pub inserts: Vec<StateInsert>,
    pub updates: Vec<StateUpdate>,
    pub removals: Vec<StateRemoval>,
}

impl StateDiff {
    pub fn canonicalize(&mut self) {
        self.inserts.sort();
        self.updates.sort();
        self.removals.sort();
    }
}
