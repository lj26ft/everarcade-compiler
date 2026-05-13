use everarcade_host::state_folder::{validation::validate, writer::initialize};
#[test]
fn creates_state_layout() {
 let base=std::env::temp_dir().join("everarcade-state-folder-test");
 let _=std::fs::remove_dir_all(&base);
 initialize(&base).unwrap();
 assert!(validate(&base));
}
