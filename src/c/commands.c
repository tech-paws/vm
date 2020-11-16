#include "types.h"

struct CommandPayload {
    u32 size;
    u8* base;
};

struct Command {
    struct CommandPayload payload;
    u32 type;
};
