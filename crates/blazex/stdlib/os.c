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

struct Person {
    int a;
};

struct Person *accept_obj(struct Person *x) {
    struct Person *r = x;
    return r;
}