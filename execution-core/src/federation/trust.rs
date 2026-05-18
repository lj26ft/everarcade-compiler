#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustDomain {
    Local,
    Federation,
    Observer,
    Validator,
}
