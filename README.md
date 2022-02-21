# Advent of Code 2021

[Advent of Code 2021](https://adventofcode.com/2021) !
I’m trying Rust this year, let’s see how it goes :)

For reference, I only know Python and I have absolutely no experience with typed languages nor with compiled languages.

# What I learned

- Day 1: Use of `cargo`, a rust project’s structure, the `cargo.toml` file and the `[[bin]]` keyword.
- Day 2: The difference between a `String` and a `str`, the `option` type and the `unwrap()` method, a first try at simple pattern matching and string splitting with `lines()` and `split_whitespace()` methods.
- Day 3: First brush with the borrow checker and lifetimes, the definition of function with the `fn` keyword, use of `Vec<&str>`.
- Day 4: Using `HashMaps`, the creation of custom `structs`, `const` and `enums`, using the `impl` and `pub` keywords, `#[derive(PartialEq)]`, `break` and `continue` statements can be scoped to target outer loops, separating code in different files and using `crate::` and `mod`, using a constructor to initiate members of our structs, the `retain` method for vectors, dereferencing a value with `*`.
- Day 5: Deriving more stuff like `Hash`, how to write and run tests, writing a function returning `Results` (and the difference between returning a result and panicking).
