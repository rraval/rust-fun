#include <stdarg.h>

typedef void (*callback_t)(const char *fmt, void *varargs);

static callback_t shim_callback;
void set_shim_callback(callback_t cb) {
    shim_callback = cb;
}

void shim(const char *fmt, ...) {
    va_list varargs;
    va_start(varargs, fmt);

    shim_callback(fmt, varargs);

    va_end(varargs);
}

void callme(void (*cb)(const char *, ...)) {
    cb("fmt %s with num %4d", "foo", 14);
}
