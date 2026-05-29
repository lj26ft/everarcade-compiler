#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentInspection { pub component: String, pub read_only: bool }

pub fn inspect_component(component: &str) -> ComponentInspection { ComponentInspection { component: component.to_owned(), read_only: true } }
