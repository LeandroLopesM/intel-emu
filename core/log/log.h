#ifndef LOG_H
#define LOG_H

#include "../../cpu/cpu.h"

#ifdef VERBOSE
#define WHEN_VERBOSE(...) __VA_ARGS__
#define VRB_EITHER(A, B) A
#else
#define WHEN_VERBOSE(...)
#define WHEN_VERBOSE(...)
#define VRB_EITHER(A, B) B
#endif

#include "../colors.h"

#ifdef VERBOSE
#   define ERROR_PREFIX  "[" RED      VRB_EITHER("ERROR", "-")  RESET "]"
#   define WARN_PREFIX   "[" YELLOW   VRB_EITHER("WARN", "!")   RESET "]"
#   define INFO_PREFIX   "[" CYAN     VRB_EITHER("INFO", ">")   RESET "]"
#   define DEBUG_PREFIX  "[" BLUE     VRB_EITHER("DEBUG", "::") RESET "]"
#   define PANIC_PREFIX  "[" BOLD_RED VRB_EITHER("PANIC", "*")  RESET "]"
#else // VERBOSE
#   define ERROR_PREFIX  RED      "-"  RESET
#   define WARN_PREFIX   YELLOW   "!"  RESET
#   define INFO_PREFIX   CYAN     ">"  RESET
#   define DEBUG_PREFIX  BLUE     "::" RESET
#   define PANIC_PREFIX  BOLD_RED "*"  RESET
#endif // VERBOSE

typedef void(*err_handler_t(int));

void info(const char* fmt, ...);
void debug(const char* fmt, ...);
void error(const char* fmt, ...);
void warn(const char* fmt, ...);
void panic(const char* fmt, ...);
void panic_ext(cpu *c, const char* fmt, ...);

#endif // LOG_H