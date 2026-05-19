#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuelLimit(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuelConsumed(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuelReport {
    pub limit: FuelLimit,
    pub consumed: FuelConsumed,
    pub exhausted: bool,
}
