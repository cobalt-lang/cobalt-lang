# v0.9.2 - 06/04/2025

Handle calculation errors in VM.

## Added

- Added checks on division and modulus to make sure right hand operator is not zero.

## Changed

- Test files are moved into the /tests directory.

# v0.9.1 - 06/03/2025

Further efficiency improvements to VM.

## Added

- The --debug flag for the `cobalt` CLI tool shows more information.

## Changed

- VM is even more efficient space wise. 

# v0.9.0 - 06/02/2025

VM is now more efficient!

## NOTE

There is still further room for efficiency! For example, left and right are popped as values from the stack very frequently for binary operations and comparisons, error messages are also pretty repetitive. These will be patched in `v0.9.1`.

## Changed

- VM is more efficient space-wise. 108 lines in src/interpreter/vm.rs were removed. You can find a comparison [here](https://github.com/cobalt-lang/cobalt-lang/compare/878909...a0bce1).

## Removed

- Removed a note from v0.8.0 as it is no longer relevant.

# v0.8.0 - 06/01/2025

Unary expressions!

## Added

- Added unary expressions, which allows for negative numbers. Such as `-42`.
- Added new `NEG` bytecode opcode, which converts a positive number into a negative one.
- Added some new TODOs!

## Changed

- Prebuilt binaries are now compressed into .tar.xz files rather than .tar.gz for Linux binaries. macOS conversion from .zip to .tar.xz is also being considered.

# v0.7.1 - 05/27/2025

Minor features and bug fixes

## Added

- Identifiers can now have numbers after the first character, example: `mycoolvar1`.
- Added dates to all the versions.

## Changes

- Minor changes to `test.cb` to show the new identifiers with numbers feature!

## Bug Fixes

- In the parser, the function `parse_variable_stmt()`, takes in a value called "mutable", which if true would make the variable constant rather than mutable. This value has been renamed to constant.
- The parser's conditions have been reworked to avoid calling values more than once unnecessarily.
- Error messages in the parser are slightly more concise.
- Remove unnecessary match statement in the bytecode generator.
- Fix minor bugs in the VM, such as an issue with LoadLocal and removing dead code, which were not going to affect the current features.

# v0.7.0 - 05/25/2025

Constant variables added, reassignment expressions added!

## Note

This update was intended to include exponents as a feature, via the `^` operator, but due to the nature of exponents, they will be later implemented via a pow function once this language has a standard library.

## Added

- Added constant variables via the keyword `const`.
- Added reassignment expressions, you can reassign variables using `x = 42`
- Added comments, you can now add comments to files using `#` and they will continue as comments until a new line.

## Changed

- cobaltc's `compile` command has been renamed to `build`
- cbproj now works under the same system as the other CLI tools.
- Binaries for CLI tools are now compiled with link time optimization (LTO).

## Bug Fixes

- You can no longer reassign variables under the same identifier twice.

# v0.6.0 - 05/24/2025

Variable declarations! Improvements to the VM and CLI tools.

## Added

- Variable declarations! They do not support types yet.
- New todos to the Todo list in README.md (in the order I prefer they be completed.)
- All of the CLI tools now have help commands. Simply use the --help flag or help command.
- New --debug flag for `cobaltc compile`, which prints the lexing, parsing, and codegen steps in detail.

## Removed

- Removed debug println statement from `utils/files_u8.rs`.
- Removed the o.cbytes file. It has been replaced by test.cbx.

## Changed

- The codegen no longer has any cloning (uses references), making it more performant and memory efficient.
- Output files from `cobaltc` now use `.cbx` as the default extension instead of `.cbytes`.
- The `compile` command for `cobaltc` now allows you to set the name of the output (via the -o/--output flag) file, it will default to the name of the .cb file it is compiling.
- test.cb's contents now contain the new variable example! When you run it using `cobalt run test.cb --debug` it should show the number 17022 somewhere.

## Bug Fixes

- Minor code refactoring in certain areas.

# v0.5.0 - 05/23/2025

New bytecode code generator + interpreter! <bt>
Essentially, the system works similar to Java.

## Added

- Code generator, compiles an AST tree into bytecode.
- Interpreter, interprets compiled bytecode (stack based system)
- Todo section to the README.md.

## Removed

- Removed the main.rs file (used for testing the lexer, parser, codegen, and VM initially), now you compile the source into bytecode with `cobaltc` and interpret it with `cobalt`.

## Changed

- Lexer now uses `.is_ascii_lowercase()`, `is_ascii_uppercase()`, and `is_ascii_digit()` functions on characters instead of manually searching ranges.
- Floating point and integer values are now seperate, not combined.
- The Windows and macOS workflows now account for the binary name changes.

# v0.4.0 - 05/16/2025

New parser implementation and lexer improvements!

## Added

- Added parser implementation (can make Binary Expressions for now)

## Changed

- Lexer is now more efficient because it does not use any more cloning. It also uses the `phf` crate to store keywords instead of regular HashMaps.

# v0.3.1 - 05/12/2025

Changes to Windows binaries

## Changed

- Windows binaries are now linked with MSVC (like in v0.2.0) rather than MinGW (like in v0.3.0), this is because the v0.3.0 binaries were bigger than expected and we'd like to keep the size at a minimum.
- .cargo/config.toml removes the x86_64-pc-windows-gnu target settings.

## Note

- This update does not change anything compared to `v0.3.0` functionality wise. If you aren't on Windows, this update will not benefit you in any way.

# v0.3.0 - 05/11/2025

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

# v0.2.0 - 05/10/2025

Implemented a lexer for Cobalt.

## Added

- Added a lexer to the language. It is currently a work in progress.
- Added this CHANGELOG.md file.

## Changed

- `cobalt version` command now prints the version specified in [Cargo.toml](./Cargo.toml).

# v0.1.0 - 05/09/2025

Initial commit