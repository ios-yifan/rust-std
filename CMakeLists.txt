cmake_minimum_required(VERSION 3.22)
project(imlibc)

set(CMAKE_CXX_STANDARD 17)

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/rust/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/rust/target)

set(IM_SOURCE_FILE ${CMAKE_SOURCE_DIR}/rust/src/lib.rs)
set(IM_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/imlib/src/lib.rs.cc)
set(IM_LIB ${CARGO_TARGET_DIR}/debug/${CMAKE_STATIC_LIBRARY_PREFIX}imlib${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${IM_BRIDGE_CPP} ${IM_LIB}
        COMMAND cargo build --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${IM_SOURCE_FILE}
        USES_TERMINAL
        COMMENT "Running cargo..."
)


include_directories(${CMAKE_SOURCE_DIR}/engine)

include_directories(${CARGO_TARGET_DIR}/cxxbridge/rust)

add_executable(imlibc main.cpp ${IM_BRIDGE_CPP})

target_include_directories(
        imlibc
        PRIVATE
        include/
        ${CARGO_TARGET_DIR}/cxxbridge/imlib/src/
)


target_link_libraries(imlibc ${IM_LIB})

set_target_properties(

        PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)

# Windows-only configuration
if(WIN32)
    target_link_libraries(cpp_with_rust userenv ws2_32 bcrypt)
    set_target_properties(
            cpp_with_rust
            PROPERTIES
            MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
            RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
            RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )
endif()
