CC=gcc
CFLAGS ?= -Wall -Wextra

BINDIR=bin
OBJDIR=$(BINDIR)/obj

EMU_DEPS = 					\
	emu.c					\
	$(OBJDIR)/core.o		\
	$(OBJDIR)/cpu.o			\
	$(OBJDIR)/repl.o

$(BINDIR)/emu: $(OBJDIR) $(EMU_DEPS)
	$(CC) $(EMU_DEPS) -o $(BINDIR)/emu $(CFLAGS)

ifdef DEBUG
CFLAGS+=-ggdb -DDEBUG=1
VERBOSE=1
endif
ifdef VERBOSE
CFLAGS+=-DVERBOSE=1
endif

include repl/repl.mk
include cpu/cpu.mk
include core/core.mk

$(OBJDIR):
	mkdir -p $(OBJDIR)