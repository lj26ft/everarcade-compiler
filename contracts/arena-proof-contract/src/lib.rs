const MAX_OUTPUT_BYTES: usize = 64 * 1024;

#[no_mangle]
pub extern "C" fn alloc(size: u32) -> u32 {
    if size == 0 || size as usize > MAX_OUTPUT_BYTES {
        return 0;
    }
    let mut buffer = Vec::<u8>::with_capacity(size as usize);
    let ptr = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    ptr as u32
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: u32, size: u32) {
    if ptr == 0 || size == 0 {
        return;
    }
    unsafe {
        drop(Vec::from_raw_parts(ptr as *mut u8, 0, size as usize));
    }
}

#[no_mangle]
pub extern "C" fn everarcade_guest_execute(_input_ptr: u32, _input_len: u32) -> u64 {
    let output = br#"{"action":"PlayerJoin+PlayerMove+ScoreUpdate","player_id":"player-1","position":{"x":0,"y":1},"score":1}"#;
    let out_ptr = alloc(output.len() as u32);
    if out_ptr == 0 {
        return 0;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(output.as_ptr(), out_ptr as *mut u8, output.len());
    }
    ((out_ptr as u64) << 32) | (output.len() as u64)
}

#[no_mangle]
pub extern "C" fn everarcade_execute(input_ptr: u32, input_len: u32) -> u64 {
    everarcade_guest_execute(input_ptr, input_len)
}
