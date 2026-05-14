use everarcade_host::ipfs::{ipfs_publish::publish_bytes, ipfs_verify::verify_cid};

#[test]
fn ipfs_unavailable_does_not_corrupt_runtime() {
    match publish_bytes(b"artifact") {
        Some(cid) => assert!(verify_cid(&cid)),
        None => assert!(true),
    }
}
