use super::{artifact::IpfsPublicationIntent, cid::local_cid_placeholder};

pub fn build_intent(artifact_root:[u8;32],artifact_path:String,local_bytes:&[u8])->IpfsPublicationIntent{
 IpfsPublicationIntent{artifact_root,artifact_path,cid:Some(local_cid_placeholder(local_bytes))}
}
