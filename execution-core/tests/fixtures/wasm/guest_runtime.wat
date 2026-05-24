(module
  (memory (export "memory") 2)
  (global $heap (mut i32) (i32.const 4096))

  (func (export "alloc") (param $len i32) (result i32)
    (local $ptr i32)
    global.get $heap
    local.set $ptr
    global.get $heap
    local.get $len
    i32.add
    global.set $heap
    local.get $ptr)

  ;; static canonical responses
  (data (i32.const 128) "{\22mutations\22:[[\22counter\22,[49]]],\22stdout\22:[\22inc\22],\22status\22:\22ok\22}")
  (data (i32.const 256) "{\22mutations\22:[[\22a\22,[49]],[\22b\22,[50]]],\22stdout\22:[\22multi\22],\22status\22:\22ok\22}")
  (data (i32.const 384) "{\22mutations\22:[],\22stdout\22:[\22noop\22],\22status\22:\22ok\22}")
  (data (i32.const 512) "{\22mutations\22:[[\22k\22,[49]],[\22k\22,[50]]],\22stdout\22:[\22dup\22],\22status\22:\22ok\22}")
  (data (i32.const 640) "{\22nope\22:1}")
  (data (i32.const 768) "{\22mutations\22:[[\22stdout\22,[111,107]]],\22stdout\22:[\22stable stdout\22],\22status\22:\22ok\22}")

  (func $has (param $ptr i32) (param $len i32) (param $c i32) (result i32)
    (local $i i32)
    (local.set $i (i32.const 0))
    (block $no
      (loop $loop
        local.get $i
        local.get $len
        i32.ge_u
        br_if $no
        local.get $ptr
        local.get $i
        i32.add
        i32.load8_u
        local.get $c
        i32.eq
        if
          i32.const 1
          return
        end
        local.get $i
        i32.const 1
        i32.add
        local.set $i
        br $loop))
    (i32.const 0))

  (func (export "everarcade_execute") (param $ptr i32) (param $len i32) (result i64)
    ;; if request contains f => fuel loop
    local.get $ptr
    local.get $len
    i32.const 102 ;; 'f'
    call $has
    if
      (loop br 0)
    end
    ;; contains i => increment
    local.get $ptr
    local.get $len
    i32.const 105
    call $has
    if (result i64)
      i64.const 549755813951 ;; 128<<32 | 56
    else
      local.get $ptr
      local.get $len
      i32.const 109 ;; m
      call $has
      if (result i64)
        i64.const 1099511627846 ;; 256<<32 | 78
      else
        local.get $ptr
        local.get $len
        i32.const 110 ;; n noop
        call $has
        if (result i64)
          i64.const 1649267441712 ;; 384<<32 | 45
        else
          local.get $ptr
          local.get $len
          i32.const 100 ;; d dup
          call $has
          if (result i64)
            i64.const 2199023255620 ;;512<<32|101
          else
            local.get $ptr
            local.get $len
            i32.const 115 ;; s stdout
            call $has
            if (result i64)
              i64.const 3298534883405 ;;768<<32|103
            else
              i64.const 2748779069450 ;;640<<32|10 malformed
            end
          end
        end
      end
    end)
)
