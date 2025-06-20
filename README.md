# Cobalt Programming Language

Welcome to the source code of the Cobalt programming language!

# Installation

There a couple different methods that you can use to install the Cobalt programming language to your machine. <br>
Skip [here](#3-install-it-via-a-package-manager) to find methods for certain Linux distros.

## 1. Prebuilt Binaries

This is the easiest method that will work on any platform.
- 1. Go to the [download](https://cobalt.devitzer.dev/download) page and select your OS and architecture and download the binaries.
- 2. Place them wherever you'd like and add them to your PATH.
- 3. You can now use the CLI tools that come with Cobalt.

## 2. Build It Yourself (DIY)

We only recommend this method if you made changes to the source code and want to test it, but it is a valid method to install Cobalt. <br>
To use this method, you first need to have Rust installed onto your machine, including Cargo.
- 1. Clone this repository: `git clone --depth=1 https://github.com/cobalt-lang/cobalt-lang`
- 2. Build the binaries and install them to your path: `cargo install --path .` **OR** Build the binaries normally: `cargo build --release`
- 3. This step is only necessary if you built the binaries normally, you can either use them from the `/target/release` directory or put them in your PATH variable.

## 3. Install it via a Package Manager

This method is another easy, and official way to download the CLI tools. It only applies to certain Linux distros as of now, but we are always looking to expand.

### Arch Linux

The CLI tools are available on the AUR, maintained officially by us. You can install them with an AUR helper by doing: `yay -S cobalt-lang` or follow the steps below to do it manually.
- 1. Clone the AUR repository: `git clone https://aur.archlinux.org/cobalt-lang.git`
- 2. Look at the PKGBUILD file if you'd like to make sure it is secure.
- 3. Make the package: `makepkg -si`

# Todo
- Add if statements and loops (goal for v0.10.0)
- Add strings. (goal for v0.11.0)
- Make VM print the address of the byte that causes an error. (goal for v0.11.0)
- Add static types for variables (goal for v0.12.0)
- Add floating point values (goal for v0.12.0)
- Add loops. (goal for v0.13.0)
- Add functions. (goal for v0.14.0)
- Try to do as little cloning as possible VM and parser do inefficient cloning.
- More descriptive and consistent errors (always room for that!)
- Positional errors (the error says what position in the file it's referring to)
- Standard library.
- Bytecode bytes to words translator (just a tool in the future, could maybe be written in Cobalt itself!)
- Catch more errors at code generation stage.
- Include basic optimization methods including but not limited to: Constant folding, constant propagation, 