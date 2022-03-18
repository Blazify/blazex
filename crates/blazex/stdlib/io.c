#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>

int print(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);
    vprintf(fmt, args);
    va_end(args);

    return 0;
}

int println(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);
    vprintf(fmt, args);
    va_end(args);

    printf("\n");

    return 0;
}

int input_int() {
    int i;
    scanf("%d", &i);
    return i;
}

float input_float() {
    float f;
    scanf("%f", &f);
    return f;
}

char input_char() {
    char c;
    scanf("%c", &c);
    return c;
}

char *input_string() {
    char *str = (char *) malloc(sizeof(char) * 100);
    scanf("%[^\n]%*c", str);
    return str;
}
