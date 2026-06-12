OBJDIR ?= bin/obj
CFLAGS ?= -Wall -Wextra -ggdb
REPL_DEPS = 				\
	$(OBJDIR)/interface.o 	\
	$(OBJDIR)/builtins.o 	\
	$(OBJDIR)/assembler.o 	\


$(OBJDIR)/repl.o: $(REPL_DEPS)
	ld --relocatable $(REPL_DEPS) -o $(OBJDIR)/repl.o

$(OBJDIR)/interface.o: repl/repl.c
	$(CC) repl/repl.c -c -o $(OBJDIR)/interface.o $(CFLAGS)

$(OBJDIR)/builtins.o: repl/builtins/builtins.c
	$(CC) repl/builtins/builtins.c -c -o $(OBJDIR)/builtins.o $(CFLAGS)

CC_DIR=repl/i8080-asm/assembler
include repl/i8080-asm/assembler/assembler.mk
