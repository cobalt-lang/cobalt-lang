# v0.5.0 (not released yet)

New bytecode code generator + interpreter! <bt>
Essentially, the system works similar to Java.

## Added

- Code generator, compiles an AST tree into bytecode.
- Interpreter, interprets compiled bytecode (stack based system) **TODO: Make arithmetic operations less repetitive (I have an idea, it will save ~180 lines of code), also once codegen is complete, convert the HashMaps for variables into vecs, more memory efficient and it'll remove the need for cloning.**

## Changes

- Lexer now uses `.is_ascii_lowercase()`, `is_ascii_uppercase()`, and `is_ascii_digit()` functions on characters instead of manually searching ranges.

# v0.4.0

New parser implementation and lexer improvements!

## Added

- Added parser implementation (can make Binary Expressions for now)

## Changed

- Lexer is now more efficient because it does not use any more cloning. It also uses the `phf` crate to store keywords instead of regular HashMaps.

# v0.3.1

Changes to Windows binaries

## Changed

- Windows binaries are now linked with MSVC (like in v0.2.0) rather than MinGW (like in v0.3.0), this is because the v0.3.0 binaries were bigger than expected and we'd like to keep the size at a minimum.
- .cargo/config.toml removes the x86_64-pc-windows-gnu target settings.

## Note

- This update does not change anything compared to `v0.3.0` functionality wise. If you aren't on Windows, this update will not benefit you in any way.

# v0.3.0

Cross-platform support!

## Added

- Added prebuilt binaries for the following platforms:
    - Linux AMD64
    - Linux ARM64
    - Linux RISCV64
    - macOS AMD64
    - macOS ARM64
- Added `:` symbol to the lexer.
- Added `.cargo/config.toml`, which is configured in favor of a Linux AMD64 host system.

# v0.2.0

Implemented a lexer for Cobalt.

## Added

- Added a lexer to the language. It is currently a work in progress.
- Added this CHANGELOG.md file.

## Changed

- `cobalt version` command now prints the version specified in [Cargo.toml](./Cargo.toml).

# v0.1.0

Initial commit