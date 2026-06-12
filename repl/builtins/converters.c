#include "../../core/colors.h"

#include "../../core/common.h"
#include "../i8080-asm/assembler/parser/util/arrays.h"
#include <stdio.h>
#include <stdlib.h>

void hex(string_arr* sa)
{
    if (sa->len != 2)
        printf(RED"+"RESET" USAGE: /hex [num]\n| Convertes number from decimal to hex");
    else printf("%#lX\n", atol(sa->items[1].items));
}

void d2b(string_arr* sa)
{
    if (sa->len != 2)
        printf(RED"+"RESET" USAGE: /d2b [num]\n| Convertes number from decimal to binary");
    int n = atol(sa->items[1].items);

    printf("0b");
    while (n > 0) {

        // storing remainder in binary array
        printf("%c", n % 2 ? '1' : '0');
        n = n / 2;
    }
    printf("\n");
}

void dec(string_arr* sa)
{
    if (sa->len != 2)
        printf(RED"+"RESET" USAGE: /dec [num]\n| Convertes number from hex to decimal");
    else printf("%ld\n", strtol(sa->items[1].items, NULL, 16));
}

void b2d(string_arr* sa)
{
    if (sa->len != 2)
        printf(RED"+"RESET" USAGE: /b2d [num]\n| Convertes number from binary to decimal");

    printf("%ld\n", strtol(sa->items[1].items, NULL, 2));
}

const char* b2s(byte n)
{
    static char out[9];
    int i = 0;

    while (n && i != 9) {
        if (n & 1)
            out[i++] = '1';
        else
            out[i++] = '0';

        n >>= 1;
    }
    out[8] = 0;
    return out;
}