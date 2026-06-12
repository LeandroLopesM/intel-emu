#include "../../core/colors.h"
#include "debug.h"
#include <stdint.h>
#include <stdio.h>

void dump_registers(cpu* c)
{
    printf(BLUE"* Core dump:\n"RESET);
    printf("| SP: %-4Xh PC: %-4Xh\n", c->rgf.SP, c->rgf.PC);
    printf("| %d%d%d%d%d%d\n", c->rgf.S, c->rgf.Z, c->rgf.I, c->rgf.P, c->rgf.CY, c->rgf.AC);
    printf("| SZIPCA\n");
    printf(BLUE"+ General registers:\n"RESET);
    printf("| PSW: %-4Xh A: %2Xh:\n", c->rgf.A, c->rgf.PSW);
    printf("| D: %2Xh E: %2Xh => DE: %-4Xh\n", c->rgf.D, c->rgf.E, c->rgf.DE);
    printf("| B: %2Xh C: %2Xh => BC: %-4Xh\n", c->rgf.B, c->rgf.C, c->rgf.BC);
    printf("| H: %2Xh L: %2Xh => HL: %-4Xh\n", c->rgf.H, c->rgf.L, c->rgf.HL);

    fflush(stdout);
}

int clamp(int a, int b, int c)
{
    return a < b? b : a > c? c : a;
}

void dump_memory(cpu* c)
{
    int min = clamp(c->rgf.PC, 0, UINT16_MAX), max = clamp(c->rgf.PC + 5, 0, UINT16_MAX);

    printf("+ Memory dump (%X:%X):\n| { ", min, max);
    for (int i = clamp(c->rgf.PC, 0, UINT16_MAX); i < clamp(c->rgf.PC + 5, 0, UINT16_MAX); ++i)
    {
        if (i == c->rgf.PC)
            printf("[%2X] ", c->memory[i]);
        else
            printf("%2X ", c->memory[i]);
    }
    printf("};\n");
    fflush(stdout);
}