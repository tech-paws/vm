#include <stdint.h>

struct CommandPayload {
    uint32_t size;
    uint8_t* base;
};

struct Command {
    struct CommandPayload payload;
    uint32_t type;
};
