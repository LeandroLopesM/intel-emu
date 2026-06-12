#include "../../cpu/decode/decode.h"
#include "../../cpu/debug/debug.h"
#include "../../cpu/cpu.h"
#include "log.h"
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

static void __log(char* prefix, const char* fmt, va_list *va)
{
    printf("\n%5s ", prefix);
    vprintf( fmt, *va);
}

void error(const char* fmt, ...)
{
    va_list va;
    va_start(va, fmt);
    __log(
        ERROR_PREFIX, fmt, &va);
    va_end(va);
}

void warn(const char* fmt, ...)
{
    va_list va;
    va_start(va, fmt);
    __log(WARN_PREFIX, fmt, &va);
    va_end(va);
}

void info(const char* fmt, ...)
{
    va_list va;
    va_start(va, fmt);
    __log(INFO_PREFIX, fmt, &va);
    va_end(va);
}

void debug(const char* fmt, ...)
{
    va_list va;
    va_start(va, fmt);
    __log(DEBUG_PREFIX, fmt, &va);
    va_end(va);
}


static void __panic(int die, const char* fmt, ...)
{
    va_list va;
    va_start(va, fmt);
    __log(PANIC_PREFIX, fmt, &va);
    va_end(va);

    if (die) exit(1);
}

void panic(const char* fmt, ...)
{
    va_list va;
    va_start(va, fmt);
    __panic(1, fmt, va);
    va_end(va);
}

void panic_ext(cpu *c, const char* fmt, ...)
{
    va_list va;
    va_start(va, fmt);
    __panic(0, fmt, va);
    va_end(va);
    dump_registers(c);
    dump_memory(c);
    dump_decoder();
    exit(1);
}