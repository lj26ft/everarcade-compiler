use crate::{merkle::{leaf_hash::leaf_hash, Hash}, sync::{SyncRequest, SyncResponse, sync_result::SyncResult}};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncTranscript {
    pub request_hash: Hash,
    pub response_hash: Hash,
    pub convergence_result_hash: Hash,
    pub transcript_root: Hash,
}

fn hash_bytes(bytes: &[u8]) -> Hash { leaf_hash(bytes) }

pub fn build_sync_transcript(request: &SyncRequest, response: &SyncResponse, result: &SyncResult) -> SyncTranscript {
    let request_hash = hash_bytes(format!("{:?}", request).as_bytes());
    let response_hash = hash_bytes(format!("{:?}", response).as_bytes());
    let convergence_result_hash = hash_bytes(format!("{:?}", result).as_bytes());
    let mut root_material = Vec::new();
    root_material.extend_from_slice(&request_hash);
    root_material.extend_from_slice(&response_hash);
    root_material.extend_from_slice(&convergence_result_hash);
    let transcript_root = hash_bytes(&root_material);
    SyncTranscript { request_hash, response_hash, convergence_result_hash, transcript_root }
}
