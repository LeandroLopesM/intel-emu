#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "../core/colors.h"
#include "../core/log/log.h"
#include "../cpu/decode/decode.h"
#include "../cpu/cpu.h"

#include "builtins/builtins.h"
#include "i8080-cc/assembler/parser/util/arrays.h"
#include "i8080-cc/assembler/encoder/encoder.h"
#include "i8080-cc/assembler/parser/parser.h"

#include "i8080-cc/core/common.h"
cpu c;

int read_line(string* s)
{
    s->len = 0;
    char buf;

    while ((buf = getc(stdin)) != '\n')
    {
        if (buf == EOF) return 0;

        push_char(s, buf);
    }

    push_char(s, 0);
    s->len--;
    return 1;
}

int start_repl()
{
    memset(&c, 0, sizeof(cpu));
    string in_raw = {0};
    size_t i = 0;

    while (1)
    {
        printf("%llu> ", i++);

        if(!read_line(&in_raw))
        {
            printf(BG_RED"\nAborted\n"RESET);
            return 1;
        }

        if (in_raw.len <= 1)
            continue;

        if (in_raw.items[0] == '/')
            parse_builtin(tokenize(in_raw.items));
        else
        {
            int ff = 0;
            comp_unit comp = parse_line(in_raw.items, &ff);
            if (ff)
                continue;
            WHEN_VERBOSE(printf(
                "\n+ CompUnit:\n| Instr %d\n| A %d\n| B %d\n| C %d\n",
                comp.type,
                comp.opA? *comp.opA : 0,
                comp.opB? *comp.opB : 0,
                comp.opC? *comp.opC : 0));

            byte b = encode(&comp);

            if (b == 0)
                continue;

            c.memory[0] = b;
            if (*comp.opA != 0)
                c.memory[1] = *comp.opA;
            if (*comp.opB != 0)
                c.memory[2] = *comp.opB;

            instruction *i = decode(&c, -1, 0);

            cpu copy = c;
            exec(&c, i);
            compare(copy, c);

            printf("+ "GREEN"Success"RESET"\n");
            c.rgf.PC = 0;
        }
    }
}