use execution_core::{merkle::receipt_merkle::receipt_root, receipt_runtime::execution_receipt::ExecutionReceipt, sync::{validate_receipt_range, ReceiptRange}};

fn receipt(id:&str,parent:Option<&str>,idx:u64)->ExecutionReceipt{ExecutionReceipt{receipt_id:id.into(),parent_receipt:parent.map(|s|s.into()),execution_root:"e".into(),state_root:"s".into(),graph_root:"g".into(),replay_root:"r".into(),timestamp_index:idx}}

#[test]
fn receipt_range_rejects_gaps(){
 let receipts=vec![receipt("r1",None,0),receipt("r2",Some("r1"),2)];
 let range=ReceiptRange{start_index:0,end_index:1,receipt_root:receipt_root(&receipts),receipts};
 assert!(!validate_receipt_range(&range));
}

#[test]
fn receipt_range_rejects_broken_parents(){
 let receipts=vec![receipt("r1",None,0),receipt("r2",Some("x"),1)];
 let range=ReceiptRange{start_index:0,end_index:1,receipt_root:receipt_root(&receipts),receipts};
 assert!(!validate_receipt_range(&range));
}
