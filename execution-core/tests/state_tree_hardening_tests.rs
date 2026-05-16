use everarcade_abi::StateChange;
use execution_core::state::{apply_diff, decode_checkpoint, decode_checkpoint_with_expected_root, encode_checkpoint, CanonicalState};

#[test]
fn test_state_root_empty() { assert_eq!(CanonicalState::default().root(), sha2::Sha256::digest([]).into()); }

#[test]
fn test_state_root_order_independent() {
    let mut a = CanonicalState::default();
    a.entries.insert(b"a".to_vec(), b"1".to_vec());
    a.entries.insert(b"b".to_vec(), b"2".to_vec());
    let mut b = CanonicalState::default();
    b.entries.insert(b"b".to_vec(), b"2".to_vec());
    b.entries.insert(b"a".to_vec(), b"1".to_vec());
    assert_eq!(a.root(), b.root());
}
#[test]
fn test_state_root_value_change_changes_root() { let mut s=CanonicalState::default(); s.entries.insert(b"k".to_vec(), b"1".to_vec()); let r=s.root(); s.entries.insert(b"k".to_vec(), b"2".to_vec()); assert_ne!(r,s.root()); }
#[test]
fn test_state_root_key_change_changes_root() { let mut s=CanonicalState::default(); s.entries.insert(b"k1".to_vec(), b"1".to_vec()); let r=s.root(); s.entries.remove(b"k1".as_slice()); s.entries.insert(b"k2".to_vec(), b"1".to_vec()); assert_ne!(r,s.root()); }

#[test]
fn test_apply_diff_insert() { let mut s=CanonicalState::default(); let r=apply_diff(&mut s,&[StateChange{key:"k".into(),before:"".into(),after:"v".into()}]).unwrap(); assert_eq!(r,s.root()); }
#[test]
fn test_apply_diff_update() { let mut s=CanonicalState::default(); s.entries.insert(b"k".to_vec(),b"v".to_vec()); apply_diff(&mut s,&[StateChange{key:"k".into(),before:"v".into(),after:"v2".into()}]).unwrap(); assert_eq!(s.entries.get(b"k".as_slice()).unwrap(),b"v2"); }
#[test]
fn test_apply_diff_delete() { let mut s=CanonicalState::default(); s.entries.insert(b"k".to_vec(),b"v".to_vec()); apply_diff(&mut s,&[StateChange{key:"k".into(),before:"v".into(),after:"".into()}]).unwrap(); assert!(!s.entries.contains_key(b"k".as_slice())); }
#[test]
fn test_apply_diff_before_mismatch_fails() { let mut s=CanonicalState::default(); let e=apply_diff(&mut s,&[StateChange{key:"k".into(),before:"x".into(),after:"y".into()}]).unwrap_err(); assert!(e.to_string().contains("field=state_before")); }
#[test]
fn test_apply_diff_duplicate_key_fails() { let mut s=CanonicalState::default(); let e=apply_diff(&mut s,&[StateChange{key:"k".into(),before:"".into(),after:"a".into()},StateChange{key:"k".into(),before:"a".into(),after:"b".into()}]).unwrap_err(); assert!(e.to_string().contains("duplicate")); }
#[test]
fn test_checkpoint_encode_decode_roundtrip(){ let mut s=CanonicalState::default(); s.entries.insert(b"k".to_vec(),b"v".to_vec()); let e=encode_checkpoint(&s).unwrap(); let d=decode_checkpoint(&e).unwrap(); assert_eq!(s,d); assert_eq!(e,encode_checkpoint(&d).unwrap()); }
#[test]
fn test_checkpoint_order_independent_encoding(){ let mut a=CanonicalState::default(); a.entries.insert(b"a".to_vec(),b"1".to_vec()); a.entries.insert(b"b".to_vec(),b"2".to_vec()); let mut b=CanonicalState::default(); b.entries.insert(b"b".to_vec(),b"2".to_vec()); b.entries.insert(b"a".to_vec(),b"1".to_vec()); assert_eq!(encode_checkpoint(&a).unwrap(),encode_checkpoint(&b).unwrap()); }
#[test]
fn test_checkpoint_root_validation(){ let s=CanonicalState::default(); let e=encode_checkpoint(&s).unwrap(); decode_checkpoint_with_expected_root(&e,s.root()).unwrap(); assert!(decode_checkpoint_with_expected_root(&e,[1u8;32]).is_err()); }
