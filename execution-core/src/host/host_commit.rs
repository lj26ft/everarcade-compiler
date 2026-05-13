pub type Hash = [u8; 32];

pub trait HostStateCommit {
    fn commit_root(&self) -> Hash;
}
