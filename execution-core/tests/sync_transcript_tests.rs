use execution_core::sync::{sync_transcript::build_sync_transcript, sync_result::SyncResult, SyncRequest, SyncResponse};
#[test] fn transcript_stable(){
 let req=SyncRequest{local_state_root:[0;32],local_replay_root:[0;32],local_receipt_root:[0;32],from_index:0,to_index:None};
 let res=SyncResponse{checkpoint:None,receipts:vec![],state_proofs:vec![],receipt_proofs:vec![],replay_proof:None};
 let out=SyncResult{converged:true,final_state_root:[0;32],final_replay_root:[0;32],final_receipt_root:[0;32],failure:None};
 assert_eq!(build_sync_transcript(&req,&res,&out).transcript_root, build_sync_transcript(&req,&res,&out).transcript_root);
}
