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
target_link_libraries(a.out ${FLEX_LIBRARIES})
