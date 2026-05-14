pub type Hash = [u8; 32];
use super::trust_record::TrustRecord; pub fn trust_record_valid(record:&TrustRecord)->bool{ record.subject_root!=[0;32] && record.provenance_root!=[0;32]}
