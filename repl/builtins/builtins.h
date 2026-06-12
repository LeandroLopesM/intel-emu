#ifndef BUILTINS_H
#define BUILTINS_H

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "../../core/colors.h"
#include "../../core/log/log.h"
#include "../../cpu/decode/decode.h"
#include "../../cpu/cpu.h"
#include "../../cpu/debug/debug.h"

#include "../i8080-cc/assembler/parser/util/arrays.h"
#include "../i8080-cc/assembler/encoder/encoder.h"
#include "../i8080-cc/assembler/parser/parser.h"

#include "../i8080-cc/core/common.h"

void dump(string_arr* sa);
void help();
void syn();
void parse_builtin(string_arr* sa);
void compare(cpu before, cpu after);
const char* b2s(byte n);

void set(string_arr* sa);

void hex(string_arr* sa);
void d2b(string_arr* sa);
void dec(string_arr* sa);
void b2d(string_arr* sa);
const char* b2s(byte n);

#endif // BUILTINS_H