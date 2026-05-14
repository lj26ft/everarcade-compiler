fn temp_path() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    p.push(format!("everarcade-test-{}", n));
    std::fs::create_dir_all(&p).unwrap();
    p
}
use everarcade_host::state_folder::node_manifest::{
    read_node_manifest, write_node_manifest, NodeManifest,
};
#[test]
fn manifest_persisted() {
    let d = temp_path();
    let m = NodeManifest::new("n");
    write_node_manifest(d.as_path(), &m).unwrap();
    assert_eq!(read_node_manifest(d.as_path()).unwrap().node_name, "n");
}
