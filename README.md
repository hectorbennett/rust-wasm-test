# rust-wasm-test
Rust webassembly test

Following https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

and 

https://rustwasm.github.io/book/game-of-life/hello-world.html

cargo install wasm-pack


questions:
 - u32
 - usize


cdylib
------
--crate-type=cdylib, #![crate_type = "cdylib"] - A dynamic system library will be produced. This is used when compiling a dynamic library to be loaded from another language. This output type will create *.so files on Linux, *.dylib files on macOS, and *.dll files on Windows.

struct
------
A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

enum
----
Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values.

trait
----

impl
----
define methods on a struct, enum or trait

usize
-----
The pointer-sized unsigned integer type.
The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.


iter()
------

cloned()
-------


match
----
rust provides pattern matching via the match keyword, which can be used like a C switch. The first matching arm is evaluated and all possible values must be covered.

.as_slice()
----

.chunks()
----

Slice Type
------