# CAIE Assembler

The instruction set is defined in [_Cambridge International AS & A Level 9618 Computer Science syllabus for examination in 2021, 2022 and 2023_](https://www.cambridgeinternational.org/images/502962-2021-2023-syllabus.pdf), pp. 18–19.

The assembly code parser will not be implemented until the syntax for specifying addresses of instructions is well-defined, as these are required for most of the code in past papers to function.

## Build

```sh
# WebAssembly
wasm-pack build

# Vite
cd www
pnpm run build
```
