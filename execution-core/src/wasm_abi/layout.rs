pub const ENTRYPOINT: &str = "everarcade_execute";
pub const ALLOC_EXPORT: &str = "alloc";
pub const MEMORY_EXPORT: &str = "memory";

#[inline]
pub fn encode_return_handle(ptr: u32, len: u32) -> u64 {
    ((ptr as u64) << 32) | (len as u64)
}

#[inline]
pub fn decode_return_handle(handle: u64) -> (u32, u32) {
    ((handle >> 32) as u32, handle as u32)
}
