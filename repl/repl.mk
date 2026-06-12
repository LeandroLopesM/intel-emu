OBJDIR ?= bin/obj
CFLAGS ?= -Wall -Wextra -ggdb
REPL_DEPS = 				\
	$(OBJDIR)/interface.o 	\
	$(OBJDIR)/compiler.o 	\


$(OBJDIR)/repl.o: $(REPL_DEPS)
	ld --relocatable $(REPL_DEPS) -o $(OBJDIR)/repl.o

$(OBJDIR)/interface.o: repl/repl.c
	$(CC) repl/repl.c -c -o $(OBJDIR)/interface.o $(CFLAGS)

CC_DIR=repl/i8080-cc/compiler
include repl/i8080-cc/compiler/compiler.mk