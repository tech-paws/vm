#include <string.h>
#include <assert.h>
#include <stdio.h>
#include "types.h"

struct RegionMemoryBuffer {
    u64 size;
    u8* base;
    umm offset;
};

struct StackMemoryBuffer {
    u64 size;
    u8* base;
    umm offset;
};

uint8_t* virtual_alloc(uint32_t size);

struct RegionMemoryBuffer create_region_memory_buffer(u64 size) {
    u8* base = virtual_alloc(size);
    struct RegionMemoryBuffer buffer;

    if (base) {
        buffer.size = size;
        buffer.base = base;
        buffer.offset = 0;
    }
    else {
        buffer.size = 0;
        buffer.base = 0;
        buffer.offset = 0;
    }

    return buffer;
}

struct RegionMemoryBuffer region_memory_buffer_emplace_region(struct RegionMemoryBuffer* where, u64 size) {
    assert(where->offset + size <= where->size);
    struct RegionMemoryBuffer buffer;

    buffer.base = where->base + where->offset;
    buffer.size = size;
    buffer.offset = 0;

    where->offset += size;

    return buffer;
}

u8* region_memory_buffer_alloc(struct RegionMemoryBuffer* buffer, u64 size) {
    assert(buffer != 0);

    if (buffer->offset + size > buffer->size) {
        return 0;
    }

    // TODO: Alignment
    u8* result = buffer->base + buffer->offset;
    buffer->offset += size;

    return result;
}

u8* region_memory_buffer_emplace(struct RegionMemoryBuffer* buffer, u64 size, u8 const* data) {
    u8* result = region_memory_buffer_alloc(buffer, size);
    memcpy(result, data, size);
    return result;
}

void region_memory_buffer_free(struct RegionMemoryBuffer* buffer) {
    buffer->offset = 0;
}
