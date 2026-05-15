use crate::network::{
    receipt_request::DistributedReceiptRequest, receipt_response::DistributedReceiptResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributedReceiptMessage {
    Request(DistributedReceiptRequest),
    Response(DistributedReceiptResponse),
}
