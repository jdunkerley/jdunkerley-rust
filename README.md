# JDunkerley rust

This is my notes and samples from learning rust

## Why Rust (Considering Rust - https://www.youtube.com/watch?v=DnT-LUQgc7s)

- Mozilla, but community driven, open-source
- "Fast, reliable, productive -- pick 3"
- Compiled, no runtime or Garbage Collection
- Strong, static typing with elaborate type system
- Imperative with some functional elements

### Features

- Modern language
  - Nice and efficient generics (get built like C++ from source into specific implementation)
- Algebraic Datatypes and pattern matching
  - Enumerations or types which contain other types
  - No nulls
  - Option type either Some<T> or None
  - Result type either Ok or Error
  - Must handle all posibilites allows for compile time catching of the issue
- Modern tooling
  - Formatter (``cargo fmt``) and Linter (``cargo clippy``)
  - Build system
    - Helpful compiler messages
  - Package management
  - Unit testing
  - Documentation (``///`` with ``cargo doc``)
    - Including have integration tests writen within the documentation
    - Tests will fail if documenation misaligned
- Great WASM support
- Works with 

### Safety by Design

- Pointers checked at compile-time
  - (some of the perks of a managed world, without the GC)
  - Ownership model
    - Only ever a single owner, no double-free
    - Either 1 mutable or many immutable (including across threads)
  - No pointers dangling post owner changes or drops
  - No use after free
- Thread-safety from types
  - Compile knows if safe for multiple threads to access value at same time
- No hidden states
  - Algebraic types above mean you can't forget null checks or ignore errors
  - Can choose to throw away

### Low-level control

- No GC or runtime (no reflection though and less powerful debugger)
- Can issue system calls (fork/exec)
- Run on system with no OS
- Free FFI calls to other languages (e.g. C, Python, Java)
  - Allow use of external code (``extern "C"``)
  - Mark blocks as a ``unsafe`` when compiler can't check them (e.g. calling C function)
- Can expose Rust code a C functions (``pub extern "C" fn``)
- Control allocation and dispatch
  - Choose how memory is allocated
  - Override the async dispatcher

*This amounts to allowing a path to move from existing codebases to Rust in stages*

### Macros

- Metaprogramming in Rust
- Ability to manipulate the AST
- Macro is written in Rust as well
- Very powerful - for example generic serialiser which will write all the serialisation walker for it

## RustBook

