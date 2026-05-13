use sha2::{Digest,Sha256};
pub fn local_cid_placeholder(bytes:&[u8])->String{format!("stub-cid-{}",hex::encode(Sha256::digest(bytes)))}
