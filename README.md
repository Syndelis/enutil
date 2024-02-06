# Enum Utility Macros

Enutil includes general utility macros for dealing with enumerators in Rust. The main objective of this crate is to provide macros that reduce boilerplate and/or repetition of (un)commonly seen patterns associated to enumerators in Rust.

## Macro Cheatsheet

| Macro       | Description |
| ----------- | ----------- |
| [EnumDeref] | Implements `Deref` and `DeretMut` for a new-type enum based on a common `Target` |

[EnumDeref]: https://docs.rs/enutil_macros/PLACEHOLDER

## Using enutil

Simply include it in your `Cargo.toml` with the `derive` feature.

- Via Cargo's CLI:

    ```sh
    $ cargo add enutil --features=derive
    ```

- Manually:

    ```toml
    # Cargo.toml
    [dependencies]
    enutil = { version = "<type latest version here>", features = ["derive"] }
    ```

---

### Acknowledgements

This crate's structure and organization has been largely inspired by that seen in [Strum](https://github.com/Peternator7/strum). Checkout that crate for other interesting and useful macros.

### Why 'enutil'?

It joins the words "Enum" and "Utility", while also sounding like the word "inútil", which is portuguese for "useless".
