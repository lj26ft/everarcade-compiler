#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allocation {
    pub offset: usize,
    pub len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicAllocator {
    cursor: usize,
}

impl DeterministicAllocator {
    pub fn new() -> Self {
        Self { cursor: 0 }
    }
    pub fn alloc(&mut self, len: usize, alignment: usize) -> Allocation {
        let align = alignment.max(1);
        let mask = align - 1;
        if self.cursor & mask != 0 {
            self.cursor = (self.cursor + mask) & !mask;
        }
        let out = Allocation {
            offset: self.cursor,
            len,
        };
        self.cursor += len;
        out
    }
    pub fn dealloc(&mut self, _allocation: Allocation) {}
}
