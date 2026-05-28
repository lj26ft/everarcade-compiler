#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionLifecycle {
    Created,
    Running,
    Recovering,
    Closed,
}
