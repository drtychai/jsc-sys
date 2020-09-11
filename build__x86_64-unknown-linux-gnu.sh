#/bin/bash
#
# Libraries required for this build:
#    make cmake clang python3-dev ruby libglib-dev
#

###############################################################################
#                                                                             #
#                           Constant Environment Vars                         #
#                                                                             #
###############################################################################
# Explicitly use LLVM toolchain
CC=$(which clang)
CXX=$(which clang++)

MAKE=$(which make)
CMAKE=$(which cmake)

# Number of parallel c/make jobs
N_JOBS=$(nproc)

# Webkit source tree implicitly set to ${JSC_SRC} ; Falls back to CARGO_MANIFEST_DIR/WebKit if unset
WEBKIT_SRC=$(if [ -n "${JSC_SRC}" ];then echo ${JSC_SRC}; else echo "${CARGO_MANIFEST_DIR}/WebKit";fi)
BUILD_DIR=${OUT_DIR}/build

# Additional libraries needed for x86_64-unknown-linux-gnu build
LINK_GLIB=$(pkg-config --cflags --libs glib-2.0)



###############################################################################
#                                                                             #
#                             Cmake Build Args                                #
#                                                                             #
###############################################################################
CXX_FLAGS="-Wall -Werror -Wunused-but-set-variable -ftree-slp-vectorize"

# Debug build flavor of JavaScriptCore
CMAKE_ARGS="-DPORT=JSCOnly"
CMAKE_ARGS="${CMAKE_ARGS} -DCMAKE_BUILD_TYPE=Debug"
CMAKE_ARGS="${CMAKE_ARGS} -DCMAKE_C_COMPILER=${CC}"
CMAKE_ARGS="${CMAKE_ARGS} -DCMAKE_CXX_COMPILER=${CXX}"
CMAKE_ARGS="${CMAKE_ARGS} -DCMAKE_EXPORT_COMPILE_COMMANDS=ON"

# Build statically using libglib
CMAKE_ARGS="${CMAKE_ARGS} -DENABLE_STATIC_JSC=ON"
CMAKE_ARGS="${CMAKE_ARGS} -DENABLE_FTL_JIT=ON"
CMAKE_ARGS="${CMAKE_ARGS} -DENABLE_TOOLS=NO"
CMAKE_ARGS="${CMAKE_ARGS} -DEVENT_LOOP_TYPE=GLib"

# Omit superfluous test code and build proper (i.e., non-shallow) archives
CMAKE_ARGS="${CMAKE_ARGS} -DUSE_CAPSTONE=NO"
CMAKE_ARGS="${CMAKE_ARGS} -DUSE_SYSTEM_MALLOC=NO"
CMAKE_ARGS="${CMAKE_ARGS} -DUSE_THIN_ARCHIVES=OFF"
CMAKE_ARGS="${CMAKE_ARGS} -DDEVELOPER_MODE=NO"
CMAKE_ARGS="${CMAKE_ARGS} -DSHOW_BINDINGS_GENERATION_PROGRESS=1"



###############################################################################
#                                                                             #
#                       Generate Makefile and Build                           #
#                                                                             #
###############################################################################
mkdir -p ${BUILD_DIR} && cd ${BUILD_DIR}
#${WEBKIT_SRC}/build
${CMAKE} -j${N_JOBS} ${CXX_FLAGS} ${CMAKE_ARGS} -S${WEBKIT_SRC} -B${BUILD_DIR}
${MAKE} -j${N_JOBS} ${LINK_GLIB}
