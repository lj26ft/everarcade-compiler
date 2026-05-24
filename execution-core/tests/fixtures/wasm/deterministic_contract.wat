(module
  (memory (export "memory") 1)
  (global $heap (mut i32) (i32.const 1024))
  (func (export "alloc") (param $len i32) (result i32)
    (local $ptr i32)
    global.get $heap
    local.tee $ptr
    local.get $len
    i32.add
    global.set $heap
    local.get $ptr)
  (func $write (param $dst i32) (param $src i32) (param $len i32)
    (local $i i32)
    (loop $l
      local.get $i
      local.get $len
      i32.ge_u
      br_if 1
      local.get $dst
      local.get $i
      i32.add
      local.get $src
      local.get $i
      i32.add
      i32.load8_u
      i32.store8
      local.get $i
      i32.const 1
      i32.add
      local.set $i
      br $l))
  (data (i32.const 0) "{\"updated_state\":{\"counter\":[50]},\"receipt\":{\"events\":[\"deterministic-stdout\"],\"state_changes\":[[\"counter\",[50]]],\"proof\":\"proof\"}}")
  (func (export "everarcade_execute") (param $ptr i32) (param $len i32) (result i64)
    (local $out_ptr i32) (local $out_len i32)
    ;; if first byte is 'b' burn fuel
    local.get $len
    i32.const 0
    i32.gt_u
    if
      local.get $ptr
      i32.load8_u
      i32.const 98
      i32.eq
      if
        (loop $burn
          br $burn)
      end
    end
    i32.const 140
    local.set $out_len
    local.get $out_len
    call 0
    local.set $out_ptr
    local.get $out_ptr
    i32.const 0
    local.get $out_len
    call $write
    local.get $out_ptr
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get $out_len
    i64.extend_i32_u
    i64.or))
