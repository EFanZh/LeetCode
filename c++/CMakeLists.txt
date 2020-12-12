cmake_minimum_required(VERSION "3.15")
project("leet-code")

# Global configuration.

if(CMAKE_CURRENT_SOURCE_DIR STREQUAL CMAKE_SOURCE_DIR)
    set(STANDALONE_PROJECT ON)
else()
    set(STANDALONE_PROJECT OFF)
endif()

if(MSVC)
    set(CMAKE_INCLUDE_SYSTEM_FLAG_C "/external:I")
    set(CMAKE_INCLUDE_SYSTEM_FLAG_CXX "/external:I")

    add_compile_options("/experimental:external" "/external:W0")
endif()

# The main library.

add_library("leet-code" INTERFACE)

if(STANDALONE_PROJECT)
    target_include_directories("leet-code" INTERFACE "include")
else()
    target_include_directories("leet-code" SYSTEM INTERFACE "include")
endif()

target_compile_features("leet-code" INTERFACE cxx_std_17)

# Tests.

if(STANDALONE_PROJECT)
    include("GoogleTest")
    find_package("GTest" CONFIG REQUIRED)

    enable_testing()

    # Common compile options.

    add_library("common-compile-options" INTERFACE)

    if(MSVC)
        target_compile_options(
            "common-compile-options"
            INTERFACE
            "/W4"
            "/we4287" # 'expression': unsigned/negative constant mismatch
            "/we4365" # 'expression': conversion from 'type' to 'type', signed/unsigned mismatch
            "/we4388" # 'token': signed/unsigned mismatch
            "/we4826" # Conversion from 'type' to 'type' is sign-extended. This may cause unexpected runtime behavior.
            "/WX"
        )
    else()
        target_compile_options(
            "common-compile-options"
            INTERFACE
            "-Wall"
            "-Wextra"
            "-Wpedantic"
            "-Wconversion"
            "-Werror"
        )
    endif()

    # Check headers.

    set(
        LEET_CODE_HEADERS
        "include/leet-code/data-structures/list-node.h"
        "include/leet-code/problem-0237-delete-node-in-a-linked-list/modify-this.h"
        "include/leet-code/problem-0321-create-maximum-number/greedy.h"
        "include/leet-code/problem-0402-remove-k-digits/greedy.h"
        "include/leet-code/problem-0402-remove-k-digits/greedy-2.h"
        "tests/leet-code/problem-0237-delete-node-in-a-linked-list/tests.h"
        "tests/leet-code/problem-0321-create-maximum-number/tests.h"
        "tests/leet-code/problem-0402-remove-k-digits/tests.h"
        "tests/leet-code/test-utilities.h"
    )

    set(CHECK_LEET_CODE_HEADERS_SOURCES)

    foreach(LEET_CODE_HEADER ${LEET_CODE_HEADERS})
        string(REGEX REPLACE "^(include|tests)/" "" RELATIVE_HEADER_PATH "${LEET_CODE_HEADER}")
        string(REGEX REPLACE "\.[^.]*$" "" BASE_PATH "${RELATIVE_HEADER_PATH}")
        set(SOURCE_FILE "${CMAKE_CURRENT_BINARY_DIR}/leet-code-check-headers/src/${BASE_PATH}.cpp")

        file(GENERATE OUTPUT "${SOURCE_FILE}" CONTENT "#include <${RELATIVE_HEADER_PATH}>")

        list(APPEND CHECK_LEET_CODE_HEADERS_SOURCES "${SOURCE_FILE}")
    endforeach()

    add_library("check-leet-code-headers" OBJECT ${CHECK_LEET_CODE_HEADERS_SOURCES})

    target_include_directories("check-leet-code-headers" PRIVATE "tests")
    target_link_libraries("check-leet-code-headers" PRIVATE "common-compile-options" "leet-code" "GTest::gtest_main")

    set_target_properties("check-leet-code-headers"
                          PROPERTIES CXX_EXTENSIONS OFF
                                     CXX_STANDARD_REQUIRED ON)

    # Test executable.

    add_executable(
        "leet-code-tests"
        "tests/leet-code/problem-0237-delete-node-in-a-linked-list/modify-this.cpp"
        "tests/leet-code/problem-0321-create-maximum-number/greedy.cpp"
        "tests/leet-code/problem-0402-remove-k-digits/greedy.cpp"
        "tests/leet-code/problem-0402-remove-k-digits/greedy-2.cpp"
        "tests/leet-code/test-utilities.cpp"
    )

    target_link_libraries("leet-code-tests" PRIVATE "common-compile-options" "leet-code" "GTest::gtest_main")

    set_target_properties("leet-code-tests"
                          PROPERTIES CXX_EXTENSIONS OFF
                                     CXX_STANDARD_REQUIRED ON)

    gtest_discover_tests("leet-code-tests")
endif()
