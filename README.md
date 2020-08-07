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

> Currently the library is evolving quickly, this example is kinda pointless
> and will need to be changed constantly. Once the library is more stable I
> will add a example to the repo.

Simple example with incrementing state.

```toml
[dependencies]
raylib = { git = "https://github.com/deltaphc/raylib-rs", branch = "master" }
abyss_engine = { git = "https://github.com/HurricaneInteractive/abyss_engine", branch master }
```

```rust
use raylib::prelude::*;
use abyss_engine::core::{Core};

struct GameState {
    pub count: i32
}

fn main() {
    Core::init()
        .game_loop(|mut core| {
            let mut state = GameState {
                count: 0
            };

            while !&core.rl.window_should_close() {
                logic(&mut core, &mut state);
            }
        })
}

fn logic(core: &mut Core, state: &mut GameState) {
    let d = &mut core.rl.begin_drawing(&core.thread);

    if d.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) {
        state.count += 1;
    }

    d.clear_background(Color::WHITE);
    d.draw_text(&state.count.to_string(), 12, 12, 20, Color::BLACK);
}
```
