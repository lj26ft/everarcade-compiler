#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {
    let mut buffer = Vec::<u8>::with_capacity(len as usize);

    let ptr = buffer.as_mut_ptr();

    std::mem::forget(buffer);

    ptr as i32
}

#[no_mangle]
pub extern "C" fn execute(ptr: i32, len: i32) -> i32 {
    unsafe {
        // Read bytes from host memory
        let input =
            std::slice::from_raw_parts(
                ptr as *const u8,
                len as usize,
            );

        // Clone + mutate deterministically
        let mut output = input.to_vec();

        output.reverse();

        // Allocate guest output memory
        let out_ptr =
            alloc(output.len() as i32);

        // Copy output into guest memory
        std::ptr::copy_nonoverlapping(
            output.as_ptr(),
            out_ptr as *mut u8,
            output.len(),
        );

        out_ptr
    }
}
