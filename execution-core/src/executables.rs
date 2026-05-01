use crate::{
    payload::Payload,
    state::ExecutionContext,
    types::Executable,
};

pub struct DynamicExecutable {
    pub payload: Payload,
    pub id: String,
}

impl Executable for DynamicExecutable {
    fn execute(&self, ctx: &mut ExecutionContext) -> Vec<u8> {
        match &self.payload {
            Payload::Const { value } => value.clone().into_bytes(),

            Payload::Concat { with } => {
                let prev = ctx
                    .state
                    .values()
                    .last()
                    .expect("missing previous node");

                let mut out = prev.clone();
                out.extend_from_slice(with.as_bytes());
                out
            }
        }
    }
}
