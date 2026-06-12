#include "builtins.h"

#include "converters.c"
#include "set.c"

extern cpu c;

void dump(string_arr* sa)
{
    if (sa->len != 2)
        goto dump_all;
    else if (strcmp(sa->items[1].items, "cpu") == 0)
        dump_registers(&c);
    else if (strcmp(sa->items[1].items, "mem") == 0)
        dump_memory(&c);
    else if (strcmp(sa->items[1].items, "pipe") == 0)
        dump_decoder();
    else goto dump_all;

dump_all:
    {
        printf(BLUE"Dumping everything\n"RESET);
        dump_registers(&c);
        dump_memory(&c);
        dump_decoder();
    }
}

void help()
{
    printf(CYAN"Available commands:\n"RESET);
    printf(" > /dump Shows the current state of CPU registers \n");
    printf(" > /set  Sets a specific register or memory position to a value\n");
    printf(" > /hex  Converts a decimal value to hexadecimal\n");
    printf(" > /b2d  Converts a binary value to decimal\n");
    printf(" > /d2b  Converts a decimal value to binary\n");
    printf(" > /help Prints the help\n");
    printf(" > /syn  Prints the syntax reference\n");
    printf(" > /inst Lists supported isntructions\n");
}

void syn()
{
    printf(CYAN"I8080-assembler syntax:"RESET" (All case-insensitive)\n");
    printf(" > "CYAN"[INSTRUCTION]"RESET" => MVI, MOV, etc.\n");
    printf(" > $"CYAN"[REGISTER]"RESET"   => $A, $HL, etc.\n");
    printf(" > "CYAN"[HEX]"RESET"h        => 123h, etc.\n");
    printf(" > "CYAN"[DECIMAL]"RESET"d    => 21d, etc.\n");
    printf(" > "CYAN"[BINARY]"RESET"b     => 0110b, etc.\n");
}

void parse_builtin(string_arr* sa)
{
    if (strcmp(sa->items[0].items, "/dump") == 0)
        dump(sa);
    else if (strcmp(sa->items[0].items, "/set") == 0)
        set(sa);
    else if(strcmp(sa->items[0].items, "/hex") == 0)
        hex(sa);
    else if(strcmp(sa->items[0].items, "/dec") == 0)
        dec(sa);
    else if(strcmp(sa->items[0].items, "/b2d") == 0)
        b2d(sa);
    else if(strcmp(sa->items[0].items, "/syn") == 0)
        syn();
    else if (strstr(sa->items[0].items, "/h") != NULL)
        help();
    else error("Unknown command %s", sa->items[0].items);
}

void compare(cpu before, cpu after)
{
    if (before.rgf.SP != after.rgf.SP)
        printf("SP %d -> %d\n", before.rgf.SP, after.rgf.SP);
    if (before.rgf.PC != after.rgf.PC)
        printf("PC %d -> %d\n", before.rgf.PC, after.rgf.PC);

    if (before.rgf.A != after.rgf.A)
        printf("A %d -> %d\n", before.rgf.A, after.rgf.A);
    if (before.rgf.S != after.rgf.S)
        printf("S %d -> %d\n", before.rgf.S, after.rgf.S);
    if (before.rgf.Z != after.rgf.Z)
        printf("Z %d -> %d\n", before.rgf.Z, after.rgf.Z);
    if (before.rgf.I != after.rgf.I)
        printf("I %d -> %d\n", before.rgf.I, after.rgf.I);
    if (before.rgf.P != after.rgf.P)
        printf("P %d -> %d\n", before.rgf.P, after.rgf.P);
    if (before.rgf.CY != after.rgf.CY)
        printf("CY %d -> %d\n", before.rgf.CY, after.rgf.CY);
    if (before.rgf.AC != after.rgf.AC)
        printf("AC %d -> %d\n", before.rgf.AC, after.rgf.AC);

    if (before.rgf.D  != after.rgf.D )
        printf("D %d -> %d\n", before.rgf.D, after.rgf.D);
    if (before.rgf.E  != after.rgf.E )
        printf("E %d -> %d\n", before.rgf.E, after.rgf.E);
    if (before.rgf.DE != after.rgf.DE)
        printf("DE %d -> %d\n", before.rgf.DE, after.rgf.DE);

    if (before.rgf.B  != after.rgf.B)
        printf("B %d -> %d\n", before.rgf.B, after.rgf.B);
    if (before.rgf.C  != after.rgf.C)
        printf("C %d -> %d\n", before.rgf.C, after.rgf.C);
    if (before.rgf.BC != after.rgf.BC)
        printf("BC %d -> %d\n", before.rgf.BC, after.rgf.BC);

    if (before.rgf.H  != after.rgf.H)
        printf("H %d -> %d\n", before.rgf.H, after.rgf.H);
    if (before.rgf.L  != after.rgf.L)
        printf("L %d -> %d\n", before.rgf.L, after.rgf.L);
    if (before.rgf.HL != after.rgf.HL)
        printf("HL %d -> %d\n", before.rgf.HL, after.rgf.HL);
}