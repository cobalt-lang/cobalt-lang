# v0.3.2

Changes to Windows binaries

## Changed

- Windows binaries are now linked with MSVC (like in v0.2.0) rather than MinGW (like in v0.3.0), this is because the v0.3.0 binaries were bigger than expected and we'd like to keep the size at a minimum.
- .cargo/config.toml removes the x86_64-pc-windows-gnu target settings.

## Note

- This update does not change anything compared to `v0.3.0` functionality wise. If you aren't on Windows, this update will not benefit you in any way.

# v0.3.1

ISSUES WITH THIS GIT TAG LEAD TO IT BEING SKIPPED.

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