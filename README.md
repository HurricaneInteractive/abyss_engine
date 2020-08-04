# Abyss Engine

This is a very simple 2D game engine based on raylib and the Rust bindings
. Probably not suited for production use and still very much a Work in Progress.

It is also a project for me to become a bit more familiar with how Rust works.

The engine by itself will be pretty barebones since raylib takes care of
 almost everything. What it will be in charge of is adding features on top of
  raylib and adding help abstractions.
  
I also want to built a editor to place game objects in a scene, this will
 generate configuration files that the engine will read to construct scenes
 . If you just want to use raylib without all this weirdness, then ignore
  this engine.
  
## Usage

Simple "Hello, World" example.

```toml
[dependencies]
raylib = { git = "https://github.com/deltaphc/raylib-rs", branch = "master" }
abyss_engine = { git = "https://github.com/HurricaneInteractive/abyss_engine", branch master }
```

```rust
use raylib::prelude::*;
use abyss_engine::{Core};

fn main() {
    // Initialises raylib
    // This will read the config files for screen size, fullscreen etc settings 
    let mut core: Core = Core::init();

    while !&core.rl.window_should_close() {
        let mut d = &mut core.rl.begin_drawing(&core.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
```
