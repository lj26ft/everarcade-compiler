use crate::scheduler::frame::ExecutionFrame;
#[derive(Clone, Debug, Default)]
pub struct RuntimeExecutionStore {
    pub frames: Vec<ExecutionFrame>,
}
impl RuntimeExecutionStore {
    pub fn persist(&mut self, frame: ExecutionFrame) {
        self.frames.push(frame);
    }
}
