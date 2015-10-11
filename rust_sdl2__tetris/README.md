## Rust & SDL2 - Tetris
This Tetris is written in Rust 1.3.0 (newer versions should also work I guess) and uses the SDL2 for input and rendering.

### How to build
You can do almost everything with the Cargo build system that is part of the Rust infrastructure.
Sadly, you first need to download the SDL2 libraries separately. You can find out how to do this [here](https://github.com/AngryLawyer/rust-sdl2).
Everything else should be handled by this command.
```
cargo run
```

### Notes
I love IDEs, so I used [RustDT](http://rustdt.github.io/) (a eclipse plugin) with together with [Racer](https://github.com/phildawes/racer) for auto-completion. Considering Rust's young age everything worked fairly well (highlighting, auto-completion, debugging).
