#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyError {
    InvalidAdvertisement,
    DuplicatePropagation,
    UnknownSource,
    EmptyRoute,
    RouteCycle,
    InvalidSubscription,
}

pub type Result<T> = std::result::Result<T, TopologyError>;
