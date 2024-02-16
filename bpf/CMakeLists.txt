cmake_minimum_required(VERSION 3.16)

project(phalanx-bpf)

include(ExternalProject)

find_program(
  CLANG_EXE
  NAMES clang
  DOC "Path to the clang executable")

if(NOT CLANG_EXE)
  message(FATAL_ERROR "[bpf] Failed to find clang executable")
endif()

find_program(
  BPFTOOL_EXE
  NAMES bpftool
  DOC "Path to the bpftool executable")

if(NOT BPFTOOL_EXE)
  message(FATAL_ERROR "[bpf] Failed to find bpftool executable")
endif()

set(LIBBPF_SRC "${PROJECT_BINARY_DIR}/libbpf-prefix/src")
set(LIBBPF_BUILD_DIR "${LIBBPF_SRC}/libbpf-build")
set(LIBBPF_INCLUDE "${LIBBPF_BUILD_DIR}/root/usr/include")
set(LIBBPF_LIB "${LIBBPF_BUILD_DIR}/root/usr/lib64/libbpf.a")

ExternalProject_Add(
  libbpf
  PREFIX "${PROJECT_BINARY_DIR}/libbpf"
  SOURCE_DIR "libbpf"
  CONFIGURE_COMMAND mkdir -p build root
  BUILD_COMMAND
    ${CMAKE_MAKE_PROGRAM} BUILD_STATIC_ONLY=y OBJDIR=${LIBBPF_BUILD_DIR}/build
    DESTDIR=${LIBBPF_BUILD_DIR}/root NO_PKG_CONFIG=1 "EXTRA_CFLAGS=-fPIC"
    "LDFLAGS=-Wl,-Bstatic" -C ${CMAKE_CURRENT_SOURCE_DIR}/libbpf/src install
    install_uapi_headers
  INSTALL_COMMAND ""
  UPDATE_COMMAND "")

install(
  FILES "${LIBBPF_LIB}"
  DESTINATION "${CMAKE_INSTALL_LIBDIR}/"
  COMPONENT "libs-deps")
install(
  DIRECTORY "${LIBBPF_INCLUDE}"
  DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}/"
  COMPONENT "libs-deps")

# Get clang bpf system includes
execute_process(
  COMMAND
    bash -c
    "${CLANG_EXE} -v -E - < /dev/null 2>&1 |
          sed -n '/<...> search starts here:/,/End of search list./{ s| \\(/.*\\)|-idirafter \\1|p }'"
  OUTPUT_VARIABLE CLANG_SYSTEM_INCLUDES_output
  ERROR_VARIABLE CLANG_SYSTEM_INCLUDES_error
  RESULT_VARIABLE CLANG_SYSTEM_INCLUDES_result
  OUTPUT_STRIP_TRAILING_WHITESPACE)

if(${CLANG_SYSTEM_INCLUDES_result} EQUAL 0)
  string(REPLACE "\n" " " CLANG_SYSTEM_INCLUDES ${CLANG_SYSTEM_INCLUDES_output})
  message(STATUS "[bpf] BPF system include flags: ${CLANG_SYSTEM_INCLUDES}")
else()
  message(
    FATAL_ERROR
      "[bpf] Failed to determine BPF system includes: ${CLANG_SYSTEM_INCLUDES_error}"
  )
endif()

# Get target arch
execute_process(
  COMMAND uname -m
  COMMAND sed "s/x86_64/x86/"
  COMMAND sed "s/aarch64/arm64/"
  COMMAND sed "s/ppc64le/powerpc/"
  COMMAND sed "s/mips.*/mips/"
  COMMAND sed "s/s390x/s390/"
  OUTPUT_VARIABLE ARCH_output
  ERROR_VARIABLE ARCH_error
  RESULT_VARIABLE ARCH_result
  OUTPUT_STRIP_TRAILING_WHITESPACE)
if(${ARCH_result} EQUAL 0)
  set(ARCH ${ARCH_output})
  message(STATUS "[bpf] Target arch: ${ARCH}")
else()
  message(
    FATAL_ERROR "[bpf] Failed to determine target architecture: ${ARCH_error}")
endif()

# Set includes and compilation flags

# Set CLANG FLAGS
set(CLANG_FLAGS "")
list(
  APPEND
  CLANG_FLAGS
  -g
  -O2
  --target=bpf
  -D__${DEBUG}__
  -D__TARGET_ARCH_${ARCH} # Match libbpf usage in `/libbpf/src/bpf_tracing.h`
  -D__USE_VMLINUX__ # Used to compile without kernel headers.
  -I${LIBBPF_INCLUDE}
  -I${CMAKE_CURRENT_SOURCE_DIR}/include
  -isystem)

# Search all bpf includes files. (we can use bpf.h files)
file(GLOB_RECURSE BPF_H_FILES ${CMAKE_CURRENT_SOURCE_DIR}/include/*.h)

# Search all bpf.c files
file(GLOB_RECURSE BPF_C_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.bpf.c)

foreach(c_file ${BPF_C_FILES})
  get_filename_component(file_stem ${c_file} NAME_WE)
  set(O_FILE ${CMAKE_CURRENT_BINARY_DIR}/${file_stem}.bpf.o)

  add_custom_command(
    OUTPUT ${O_FILE}
    COMMAND ${CLANG_EXE} ${CLANG_FLAGS} ${CLANG_SYSTEM_INCLUDES} -c ${c_file} -o
            ${O_FILE}
    VERBATIM
    DEPENDS ${c_file} ${BPF_H_FILES}
    COMMENT "[bpf] Building BPF object: ${O_FILE}")

  list(APPEND BPF_OBJECT_FILES ${O_FILE})
endforeach()

# Generate the combined object file
set(BPF_OBJECT_FILE ${CMAKE_CURRENT_BINARY_DIR}/phalanx.bpf.o)
add_custom_command(
  OUTPUT ${BPF_OBJECT_FILE}
  COMMAND ${BPFTOOL_EXE} gen object ${BPF_OBJECT_FILE} ${BPF_OBJECT_FILES}
  VERBATIM
  DEPENDS ${BPF_OBJECT_FILES}
  COMMENT "[bpf] Linking ${BPF_OBJECT_FILE}")

# generate the skeleton
set(BPF_SKELETON ${CMAKE_CURRENT_BINARY_DIR}/phalanx.bpf.h)
add_custom_command(
  OUTPUT ${BPF_SKELETON}
  COMMAND bash -c
          "${BPFTOOL_EXE} gen skeleton ${BPF_OBJECT_FILE} > ${BPF_SKELETON}"
  VERBATIM
  DEPENDS ${BPF_OBJECT_FILE}
  COMMENT "[bpf] Generating skeleton: ${BPF_SKELETON}")

add_custom_target(phalanx-bpf-skeleton ALL DEPENDS ${BPF_SKELETON})
