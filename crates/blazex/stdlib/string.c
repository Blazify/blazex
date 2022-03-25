#include <string.h>
#include <stdlib.h>

char *str_concat(char *s1, char *s2) {
    char *s = malloc(strlen(s1) + strlen(s2) + 1);
    strcpy(s, s1);
    strcat(s, s2);
    return s;
}

char get_char_at(char *s, int i) {
    return s[i];
}