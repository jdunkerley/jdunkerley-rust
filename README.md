# JDunkerley rust

This is my notes and samples from learning rust

## Why Rust (Considering Rust - https://www.youtube.com/watch?v=DnT-LUQgc7s)

- Mozilla, but community driven, open-source
- "Fast, reliable, productive -- pick 3"
- Compiled, no runtime or Garbage Collection
  - All the power of the machine
- Strong, static typing with elaborate type system
- Imperative with some functional elements
- Goals: Safety, Productivity, Control

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
- Works with exisitng low level tooling
- Built in support for ``async``/``await`` with a lot of flexibility

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

### Drawbacks

- Steep learning curve (especially borrow checker)
- Young ecosystems - growing quickly
- A lot of compile time (as building generic etc, not prebuilt libraries)
- Vendor support
- Windows support

## RustBook

### Tooling

- `rustc`: Rust compiler
  - `rustc fmt` format code
  - `rustc clippy` lint code
  - `rustc doc` generate documentation
- `cargo` - Rust package manager and ofter controller of `rustc`
  - `cargo new` create new project
  - `cargo build` build project (`cargo b`)
    - `--release` build with optimizations
  - `cargo check` compile and verify but no output build (`cargo c`)
  - `cargo run` run project (`cargo r`)
  - `cargo test` run tests (`cargo t`)
  - `cargo bench` run benchmarks
  - `cargo doc` generate documentation

## Variables

- Immutable by default
  - `let a = 1;`
  - `let mut a = 1;` - mutable variable `a`
- `const` must be typed and upper case name
- `drop` keyword to free memory and release the variable
- Shadowing if define a new x in the same scope will hide old x
  - `let x = 1; let x = 2;`
  - Does not need to be the same type
- Integer Types:
  - Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (used for pointer size or indexing collections)
  - Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
  - Literals `0x` (hex), `0o` (octal), `0b` (binary), `_` (spacer), `b'A'` (u8 byte literal)
    - Can include a type suffix, e.g. `let a = 3i32;`
  - Integer overflow only at DEBUG (generates a Panic at runtime)
    - `checked_add` and `checked_sub` will return None if overflow
    - `wrapped_add` and `wrapped_sub` will wrap around at the bounds
    - `overflowing_add` and `overflowing_sub` will return the wrapping value and a boolean indicating overflow
    - `saturating_add` and `saturating_sub` will saturate at the bounds
- Floats: `f32`, `f64`
- **All numeric type conversion is explicit**
  - Comparisons must be on same types
  - `as` keyword for casting to a type
- Boolean: `bool` - `true` / `false`
- Characters: `char`
  - 4 byte literal
  - Unicode by default
- Void type: `()` called *unit*
- Tuples:
  - Do not need to be the same type
  - Defined between `()`
  - Built in destructoring:
    - `let (x, y) = (1, 2);`
  - Access via `.` index: `let x = (1, 2); x.0`
- Arrays:
  - Fixed size
  - On stack, not heap
  - let a: [i32; 5] = [1, 2, 3, 4, 5];
  - Access via `[]` index: `let a = [1, 2, 3, 4, 5]; a[0]`
    - Index is a `usize`
  - Access via `..` range: `let a = [1, 2, 3, 4, 5]; a[..]`
    - Range is a `Range<usize>`
    - Inclusive of start, and exclusive of end
    - Drop first index implies `0`, drop last index implies `len`
- Strings:
  - Can have string literals as well but they are immutable
    - Hardcoded into the compiled output
    - Create String from literal: `let s = String::from("Hello!");`
  - Unicode
  - UTF-8
    - bytes not aligned to characters
  - Heap allocated
  - Can be mutated

## Complex Types

- No constructors but everything has a literal form
- By convention types often have a `new` method
- `.` to access properties
- A `struct` is similar to an object
  - No inheritance, some capabilities with traits
  - Like a tuple but with named elements
  - Entire instance is mutable or not
- Creating
  - Use field init syntax `Complex { real: 1, imaginary: 2 }`
  - If variable matches field name can avoid duplicating
  - `..` to expand an existing struct copying to new one
    - This will move the values into the new struct
- Can create without field names - *struct tuple* 
  - `struct Color(i8, i8, i8);`
- Can also create a *unit-like struct*: `struct UnitLikeStruct;`
- Methods for a struct are implemented inside an `impl` block with same name
  - For instance methods, first parameter is `&self`, short for `self: &Self`
    - Like parameters can take ownership (`self`), borrow (`&self`) or mutable borrow (`&mut self`)
  - If no `self` parameter, then becomes like a static method
  - Can overlap field names (compiler will choose based on context)
  - Can have multiple `impl` blocks
- Traits are like interfaces
  - 

## Functions

- Rust is an expression language
  - No need for a `return` at end
    - last expression is automatically returned
    - Don't put a semi-colon after it
- Function definition
  - Like C#, parameters are passed by value if stack, by ref on heap
  - Parameters must be typed
  - Return type must be typed
  - Definition: `fn add(a: i32, b: i32) -> i32`
- Entry point is `fn main()`
- `!` at the end of a function name denotes a macro
  - `println!` is a macro
    - `{}` for placeholders, prints via `std::fmt::Diaplay` trait
    - `{:?}` for debug
    - `{:b}` for binary, `{:x}` for hex,  `{:o}` for octal
    - `{:?}` prints via `Debug` trait, `{:#?}` pretty prints this
      - `#[derive(Debug)]` will auto implement a basic `Debug`
      - `dbg!(&rect)` will print out the structure and return ownership
- Lifetimes
  - Passed in like generic but with a leading `'`
  - Specified the lifetime of 1 or more references passed in
- Generics
  - Capital letter for the type
    - `add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T`
  - Where clause can be used to apply additional constraints

## Modules

- Dependencies listed in the `cargo.toml` file
  - `cargo add` to add using utility 
- `use` to bring into local scope
- `::` to move into namespace / type

## Control Flow

- `if` is an expression so can be a tenary
  - Note no need for brackets on condition
  - `if a { b } else { c }`
  - Can include unwrapping:
    - `if let Some(x) = a { x } else { -1 }`
- `match` - pattern matching
  - like a `switch` statement but must cover every case
  - single values, ranges (with `..`), lists (with '|') and catch all (`_`)
- `for a in container`
  - container can't be accessed post loop, use a reference if still needed
  - Make a mutable reference if need to alter item (`&mut collection`)
  - Behind the scenes becomes
    - `collection` ==> `IntoIterator::into_iter(collection)`
    - `&collection` ==> `collection.iter()`
    - `&mut collection` ==> `collection.iter_mut()`
  - Use `_` if you don't want the value of item (e.g. loop over 10 times)
  - Avoid old style for loop to access collection items (performance, safety)
- `loop`
  - Is an expression with a return value
  - Infinite loop
- `while`
  - Is an expression with a return value
  - As with if no brackets on condition
- `continue` to skip
- `break` to exit loops
  - Can take an expression to return the value from the loop
  - Can also target labels: `break 'outer;` and then mark loop with `'outer:`

## Ownership

- Rust uses ownership to manage memory
  - No GC to automatically clear up
  - No need to allocate and free
- Stack vs Heap
  - Stack
    - A pile of plates
    - Last In First Out
    - Adding (pUshing) / Removing (popping)
    - All data on stack is fixed size
    - No need to hunt for space so faster to allocate
  - Heap
    - Allows for dynamic allocation
    - Pointer to the allocated memory
    - Less organised
    - Accessing memory is slower as have to follow pointer
    - Keeping track of what is using data and cleaning up is what ownership addresses
- Every value has a variable that is its owner
  - There can only be one owner
  - When owner goes out of scope, value is dropped
  - The special function `drop` is called to release it automatically
- Consider assignment: `let b = a;`
  - Stack the value would be copied if `Copy` trait
    - NUmerics, char, bool
    - Tuples if they contain copyable types
    - *Not arrays*
  - For heap the value would be *moved* and `b` is the owner
    - `a` is no longer valid
    - When `b` goes out of scope, it will be dropped
    - To copy use the `.clone()` method
      - deep copy
- Function parameters will act as above
  - Passing a String would move the ownership
  - It would be dropped when function ends
  - Returning values likewise acts as above
- References and Borrowing
  - If a Type is prefixed with `&` it is a reference
  - Passed in with an `&`
  - `&mut` is mutable reference
    - Makes it very apparent if a pure function or not
    - If a mutable reference, no other references at same time
    - Define extra scopes as needed (`{}`)
    - Compile (since 2018) will scope references releasing after  last time used
  - Ownership not moved
  - Opposite of `&` is `*` (dereferencing)
  - Reference cant live longer than owner (compile error)
- String Slice `&str`
  - Borrows part of a String
  - Protects against string mutation
  - String literals are `&str`
  - Use in preference to `String` for immutable inputs
- Similar slices for other array types