# word-square-rs
An implenentation of a backtacking algorithm to find word squares and rectangles from a given list of words. The implementation was inspired and losely based on [HackerPoet/WordSquares](https://github.com/HackerPoet/WordSquares), but is more general with respect to languages (works with any arbitrary character set) and implements a basic CLI and multithreading. Without multithreading word-square-rs performs comparably to the originaal C++ code depsite using hash tables and therefore having to perform hash lookups intead of just doing array indexing (completely thanks to the insane performance of [gxhash](https://github.com/ogxd/gxhash)).

## Running
Running the program is as simple as `cargo run -r -- <args>` with a compatible Rust version (tested on 1.84.1, but should work some older versions). For help with the arguments simply pass the argument `--help`.

### Nix
If you have nix installed, the following will let you run the code without cloning this repo:
```
nix run github:xyven1/word-square-rs -- <args>
```
