use crate::stable_hash;

pub fn asset_hash(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    stable_hash(&["asset", &text])
}
