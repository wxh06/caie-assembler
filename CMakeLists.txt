cmake_minimum_required(VERSION 3.15)

include(FetchContent)

FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.4 # Optionally specify a commit hash, version tag or branch here
)
# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH Cargo.toml)

corrosion_experimental_cbindgen(TARGET caie-assembler HEADER_NAME assembler.h FLAGS --lang c)


find_package(BISON REQUIRED)
find_package(FLEX REQUIRED)

BISON_TARGET(Parser parser.y ${CMAKE_CURRENT_BINARY_DIR}/y.tab.c)
FLEX_TARGET(Scanner lexer.l ${CMAKE_CURRENT_BINARY_DIR}/lex.yy.c)
ADD_FLEX_BISON_DEPENDENCY(Scanner Parser)

include_directories(${CMAKE_CURRENT_BINARY_DIR})
add_executable(a.out
        ${BISON_Parser_OUTPUTS}
        ${FLEX_Scanner_OUTPUTS}
)

# In this example the the `Cargo.toml` file passed to `corrosion_import_crate` is assumed to have
# defined a static (`staticlib`) or shared (`cdylib`) rust library with the name "rust-lib".
# A target with the same name is now available in CMake and you can use it to link the rust library into
# your C/C++ CMake target(s).
target_link_libraries(a.out ${FLEX_LIBRARIES} caie-assembler)
