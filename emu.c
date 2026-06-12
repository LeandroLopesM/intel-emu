#include "repl/repl.h"

int main(int argc, char** argv)
{
    (void)(argv);
    if (argc == 1)
        return start_repl();
}