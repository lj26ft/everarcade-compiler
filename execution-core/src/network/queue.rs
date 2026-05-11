use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct ExecutionQueue {
    queue: VecDeque<String>,
}

impl ExecutionQueue {
    pub fn push(&mut self, package_id: String) {
        self.queue.push_back(package_id);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
