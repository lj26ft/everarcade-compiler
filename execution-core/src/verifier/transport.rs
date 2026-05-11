use super::sync::SyncObject;

pub trait Transport {
    fn broadcast(&mut self, object: SyncObject);
    fn drain(&mut self) -> Vec<SyncObject>;
}

#[derive(Default)]
pub struct LocalTransport {
    queue: Vec<SyncObject>,
}

impl Transport for LocalTransport {
    fn broadcast(&mut self, object: SyncObject) { self.queue.push(object); }

    fn drain(&mut self) -> Vec<SyncObject> { std::mem::take(&mut self.queue) }
}
