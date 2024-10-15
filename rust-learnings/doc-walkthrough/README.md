# Notes

## Chapter 1

- Cargo is rust's build system and package manager
- `cargo new <project_name>` to initialize a project
- Cargo.toml is our project configuration file
- Source files should be inside the `src` folder
- `cargo build` to build the executable (in debug mode)
- `cargo run` to build and run at once, doesn't rebuild if files haven't changed (hence this is also a build system)
- `cargo check` to verify if there are changes made to the file
- `cargo build --release` to build executable (in release mode)

## Chapter 2

- Add module and version (semantic versioning) to the dependency list and rust will take care of it for you.
```toml
[dependencies]
rand = "0.8.5"
```

- Packages are pulled from Crates.io
- Binary crates are executable; library crates contain code that is intended to be used in other programs and can't be executed on their own
- `cargo update --verbose` to update your packages: however, cargo will only update if a better patch version is available and not on major and minor versions (since it might break the code)
- `cargo doc --open`command will build documentation for all dependencies and open it in the browser (mind blown!)


## Chapter 3

- Mutability is a strong concept because if you reason that some value should not change, then no portion of the code following the variable declaration should be allowed to mutate the value.
- Constant naming convention is all upper case separated by underscores. 
- Constant evaluation means you don't always have to compute constant values beforehand (for eg, `const X = 3 * 60 * 60`). They are not guaranteed to be evaluated at compile time.
- Receive warnings on unused variables (but it is not as strict as in Go).
- Scalar types (single value types) - integer, floats, bools, chars
- Rust's char type is four bytes in size (to support unicode).
- Expressions implicitly return the `()` value, which is also called the unit tuple or simply the unit.
- In out of index memory access, rust panics and immediately exits, thus preventing invalid memory access.
- Statements vs Expressions.
    - Statements are instruction that perform some action and do not return a value. Assignment is a statement and it doesn't return a value, so you can not do x = y = 6 since (y = 6) does not return a value.
    - Expressions evaluate to a resultant value. Expressions DO NOT include ending semicolons.


### Chapter 4
- Stack vs Heap: All data stored on stack must have a known size at compile time, whereas if size is not known in compile time, it may be stored in the heap. Pointer to the heap can be stored on the stack, but when trying to access heap memory you must follow the pointer value.
- The contents of a string literal are known at compile time and the text is directly hardcoded into the final executable, hence why string literals are fast and efficient.
- Memory allocated on the heap is "freed" as soon as the variable that "owns" it goes out of scope (RAII in C++ - deallocating resources at the end of an item's lifetime).