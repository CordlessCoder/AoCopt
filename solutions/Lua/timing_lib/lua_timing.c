#ifdef __cplusplus
#include "lua.hpp"
#else
#include "lauxlib.h"
#include "lua.h"
#include "lualib.h"
#endif

#ifdef __cplusplus
// Needed for C++ name mangling to not ruin our lives
extern "C" {
#endif
#include <math.h>
#include <time.h>

static int time_ns(lua_State *L) {
    struct timespec time;
    clock_gettime(CLOCK_MONOTONIC, &time);

    long long int nanos = time.tv_sec * 1000000000 + time.tv_nsec;

    lua_pushinteger(L, nanos);
    return 1;
}

static const struct luaL_Reg timing[] = {
    {"time_ns", time_ns}, {NULL, NULL} /* sentinel */
};

int luaopen_timing(lua_State *L) {
    luaL_newlib(L, timing);
    return 1;
}

#ifdef __cplusplus
}
#endif
