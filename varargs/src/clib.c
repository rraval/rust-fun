#include <stdarg.h>

typedef void (*callback_t)(int num, void *varargs);

static callback_t shim_callback;
void set_shim_callback(callback_t cb) {
    shim_callback = cb;
}

void shim(int num, ...) {
    va_list varargs;
    va_start(varargs, num);

    shim_callback(num, &varargs);

    va_end(varargs);
}

void va_arg_int(va_list *varargs, int *arg) {
    *arg = va_arg(*varargs, int);
}

void callme(void (*cb)(int, ...)) {
    cb(3, 13, 42, 57);
}
