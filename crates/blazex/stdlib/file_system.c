#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define _CRT_SECURE_NO_WARNING


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
