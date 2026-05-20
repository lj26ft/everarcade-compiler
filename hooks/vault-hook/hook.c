#include <stdint.h>

extern uint32_t _g(uint32_t id, uint32_t reserved);

int64_t hook(uint32_t reserved) {
    (void)reserved;
    _g(1, 0);
    return 0;
}

int64_t cbak(uint32_t reserved) {
    (void)reserved;
    _g(1, 0);
    return 0;
}
