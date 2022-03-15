#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>
#include <sys/utsname.h>

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

char *read_file(const char *path) {
    FILE *fp = fopen(path, "r");
    if (fp == NULL) {
        return NULL;
    }

    fseek(fp, 0, SEEK_END);
    long size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    char *content = malloc(size + 1);
    fread(content, 1, size, fp);
    content[size] = '\0';

    fclose(fp);

    return content;
}

int write_file(const char *path, const char *content) {
    FILE *fp = fopen(path, "w");
    if (fp == NULL) {
        return -1;
    }

    fwrite(content, 1, strlen(content), fp);
    fclose(fp);

    return 0;
}

int delete_file(const char *path) {
    return remove(path);
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

char *platform() {
#ifdef _WIN32
    return "win32";
#elif _WIN64
    return "win64";
#elif __APPLE__ || __MACH__
    return "darwin";
#elif __linux__
    return "linux";
#elif __unix__
    return "unix";
#elif __posix
    return "posix";
#elif  __FreeBSD__
    return "freebsd";
#elif __OpenBSD__
    return "openbsd";
#elif __NetBSD__
    return "netbsd";
#elif __DragonFly__
    return "dragonfly";
#elif __sun
    return "sunos";
#else
    return "unknown";
#endif
}