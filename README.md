# Cobalt Programming Language

Welcome to the source code of the Cobalt programming language!

Right now, there is not much to look at, and it's not meant to be downloaded. It is simply to show active steps in development.

# Todo
- Add floating point values (goal for v0.8.0)
- Add negative numbers (goal for v0.8.0)
- Make VM more space efficient, as binary expression opcode matching takes up a lot of unnecessary space, I have an idea to shorten this by around 200 lines. (goal for v0.9.0)
- Add more debug information to the VM when running with the --debug flag, such as variable stack. (goal for v0.9.0)
- Add strings. (goal for v0.10.0)
- Add static types for variables (goal for v0.11.0)
- Add functions. (goal for v0.11.0)
- Try to do as little cloning as possible VM and parser do inefficient cloning.
- More descriptive and consistent errors (always room for that!)
- Positional errors (the error says what position in the file it's referring to)
