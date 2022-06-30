# Logbook: Log 01

A port of the _RoguelikeDev Does The Complete Roguelike Tutorial_, edition 2022, with Rust and macroquad 😊.

| Item           | Link                                                                       |
|----------------|----------------------------------------------------------------------------|
| Tutorial       | [Part 01](https://rogueliketutorials.com/tutorials/tcod/v2/part-1/)        |
| Macroquad docs | [0.3.20](https://docs.rs/macroquad/0.3.20/macroquad/index.html)            |
| Repo           | [#commithashtodo](https://github.com/foresterre/ronguelike-tutorial-2022#) |
| Demo           | [TODO WASM](https://foresterre.github.io/ronguelike-tutorial-2022/part_01) |


## Introduction

Welcome to the first logbook of my Roguelike Tutorial 2022 implementation.

In this logbook, I aim to document my experience in participating in the 2022
edition of the [RoguelikeDev Does The Complete Roguelike Tutorial](https://old.reddit.com/r/roguelikedev/comments/vhfsda/roguelikedev_does_the_complete_roguelike_tutorial/).

While the official source will use Python with libctod, I'll be using [macroquad](https://github.com/not-fl3/macroquad) instead.

To start, I'll be first following the macroquad getting-started guide from its readme.
From here, I'll continue with the official week RonguelikeDev week 01 tutorial.

Let's get started!

### Setting-up macroquad

I'll assume [Rust](https://www.rust-lang.org/learn/get-started) itself is present on the system, and we'll also use Cargo, which should be included as a default component
of your Rust toolchain.

```bash
$ rustc --version
rustc 1.61.0 (fe5b13d68 2022-05-18)
$ cargo --version
cargo 1.61.0 (a028ae42f 2022-04-29)
```

First, create a new rust project with Cargo:

```bash
$ cargo new --bin roguelike-tutorial-2022
```

Then, to add `macroquad` as a dependency:

```bash
$ cd roguelike-tutorial-2022
$ cargo add macroquad
   Updating 'https://github.com/rust-lang/crates.io-index' index
      Adding macroquad v0.3.20 to dependencies.
             Features:
             + audio
             + quad-snd
             - backtrace
             - log
             - log-rs
```

As suggested by [macroquad](https://github.com/not-fl3/macroquad#setting-up-a-macroquad-project)'s getting-started guide, 
let's add some code in `src/main.rs` to determine whether our setup works:

```rust
use macroquad::prelude::*;

#[macroquad::main("Crabby Cave")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("CRABBY CAVE WHOOP WHOOP!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
```

If you compile and run the program, a window should appear containing a line, a rectangle, a circle and the text "CRABBY CAVE WHOOP WHOOP!",
all rendered on top of a red background. Each of the respective render functions are defined by `macroquad` in the `prelude` module which
is imported at the top of our file. We also already have a main render loop `loop { ... }`, in which we draw a series of
graphical elements. At the end of the loop scope, we wait for the rendering to finish drawing the frame with the `next_frame`
function. Since we're using `async` rust for just the main loop, we have to await this rendering function.

The `macroquad` author explains in more detail why we're using `async` for the main loop [here](https://github.com/not-fl3/macroquad/tree/04a53d07976337c212656a897ec93cee75d006d2#asyncawait).

### Drawing the main character

Now that we can open a window, and saw that we can draw elements to the screen. Let's remove the example code and get back
to a blank slate. I also changed the background from red to black.

```rust
#[macroquad::main("Crabby Cave")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
```

Our roguelike doesn't work without a main character. The graphics we'll use to represent this character is
the `@` symbol from the `dejavu10x10_gs_tc.png` image. This image is a tileset and contains many textures in one.
To load this tileset, and load each texture separately, we'll use another crate written by the `macroquad` authors:
`macroquad-tiled`. Let's add it to our game:

```bash
$ cargo add macroquad-tiled
    Updating 'https://github.com/rust-lang/crates.io-index' index
      Adding macroquad-tiled v0.1.1 to dependencies.
```

One of the things I truly love about the Rust ecosystem is [docs.rs](docs.rs). All crates published to [crates.io](crates.io) have their documentation published on docs.rs. Even if the author does not document their code, the docs are still very useful to find the right functions, traits, implementations etc.

Let's go to the docs.rs page of `macroquad-tiled`: https://docs.rs/macroquad-tiled/0.1.1/macroquad_tiled/index.html. To search, we can press <kbd>S</kbd>. Keyboard shortcuts and some tips how to navigate around effectively can be found by pressing <kbd>?</kbd>. One of my favorites is the option to search by type signature, e.g. `* -> Map` (we don't care what parameters the function should have, but it should return a `Map`).

A quick inspection of the `macroquad-tiled` index page suggests that we could use the [load_map](https://docs.rs/macroquad-tiled/0.1.1/macroquad_tiled/fn.load_map.html) function. Looking at the documentation, there's a bit of a setback. The first argument should be the contents of a "json tileset" file. Since we really only need the tileset to be divided in squares of 10x10, creating a json tileset felt overkill. I considered to instead subdivide the tileset into separate textures without the `macroquad-tiled` library, but a quick search revealed that the [Tiled](https://www.mapeditor.org/) editor can create a json tileset in seconds. 

To do so, download and install Tiled, then go to `File` -> `New` -> `New Tileset`, enter a name (I picked `roguelike_tileset`). Select `dejavu10x10_gs_tc.png` from the `assets` folder as the Image Source. Then set the Image Tile width and Image Tile Height to both 10px. Both the Image Margin and Image Spacing can be kept at 0px. Then press `Save as` and save it to our assets folder as `dejavu10x10_gs_tc.tsj`. Before we close Tiled, ...

TODO the Tiled export does not work...
