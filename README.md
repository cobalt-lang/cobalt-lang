# Cobalt Programming Language

Welcome to the source code of the Cobalt programming language!

Right now, there is not much to look at, and it's not meant to be downloaded. It is simply to show active steps in development.

# Todo

- Add support for variable declarations (goal for v0.6.0) 
- Make VM more space efficient, as binary expression opcode matching takes up a lot of unnecessary space, I have an idea to shorten this by around 200 lines.
- Allow for users to deifne the name of the output file when running `cobaltc compile`.
- Try to do as little cloning as possible, currently the code generator, VM, and parser do inefficient cloning.
- More descriptive and consistent errors (always room for that!)