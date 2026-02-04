# Graphicility: A Minimal Graphics Library

Graphicility is a minimal graphics library that makes drawing pixels to a window simple.

If you've ever tried to draw pixels to a window in Rust for example, for a CHIP-8 emulator you know the pain. Using `iced` or `egui` often feels like overkill, while `wgpu` + `winit` feels far too low-level.

**_Graphicility_ exists to bridge that gap.**

It provides a small, immediate-mode drawing API on top of a window and render loop, without forcing a UI framework or architectural style.

---

## Graphicility is not!

- A UI framework
- A widget or layout system
- A game engine
- An alternative for `iced` or `egui`
- A replacement for `wgpu`

If you need those things, Graphicility is probably not the right tool and that’s okay, it doesn't have to be.

---

## Features

- ✅ Simple pixel and shape drawing (rectangles, pixels)
- ✅ Built-in 8×8 bitmap font for text rendering
- ✅ Automatic DPI scaling
- ✅ Fixed logical resolution (perfect for pixel-art games and emulators)
- ✅ Resizable windows with aspect ratio preservation
- ✅ Zero-setup: just call `run()` and start drawing

## Example

### Basic Drawing

```rust
use graphicility::{run, Color};

fn main() {
    run(|ctx| {
        let g = ctx.graphics();
        g.clear(Color::WHITE);
        g.rect(50, 50, 100, 100, Color::BLACK);
    });
}
```

![drawing](./assets/drawing_demo.png)

### And A bouncing Rect

```rust
use graphicility::Color;
fn main() {
    let mut pos = Vec2 { x: 50, y: 50 };
    let mut vel = Vec2 { x: 2, y: 3 };
    let size = Vec2 { x: 20, y: 20 };

    graphicility::run_with(conf, move |ctx| {
        let dt = ctx.delta_time();
        let g = ctx.graphics();

        pos.x += (vel.x as f64 * dt * 60.0) as i32;
        pos.y += (vel.y as f64 * dt * 60.0) as i32;

        let (width, height) = g.logical_size();

        if pos.x <= 0 || pos.x + size.x >= width as i32 {
            vel.x = -vel.x;
        }
        if pos.y <= 0 || pos.y + size.y >= height as i32 {
            vel.y = -vel.y;
        }

        g.clear(Color::WHITE);

        g.rect(pos, size, Color::rgb(128, 23, 255));

        g.text((10, 10), "Graphicility v0.3.0", Color::CYAN);
        g.text((10, height as i32 - 20), format!("Pos: {}, {}", pos.x, pos.y), Color::BLACK);

    });
}
```

![demo](./assets/graphliclity_demo.gif)

---

## When to use Graphicility?

- Developing a Emulator(Especially a Retro one) = Perfect ✅
- Developing Software in Rust that needs basic Graphical Output = Perfect ✅
- Developing a UI Framework taking Graphicility as a foundation = Perfect! See [Developing Extensions](./DEVELOPING_EXTENSIONS.md) ✅
- Developing a Full fledged Game or a Desktop Application = Nope ❌ You'll struggle and will need to implement most of the things by yourself

---

## How it works

Internally, Graphicility handles:

- Window creation and events
- A simple render loop
- Pixel drawing
- DPI Controls

You control **what** is drawn.
Graphicility handles **how** it appears on screen.

---

## Philosophy

Graphicility is designed to be the **first mile** of graphics in Rust.

It favors:

- Explicit code over magic
- Simplicity over complexity
- Approachability over completeness

Drawing a rectangle should not require hundreds of lines of setup.

---

## Limitations

- **No low-level access**: Graphicility intentionally hides renderer internals to keep things simple
- **No audio**: Graphicility is graphics-only
- **Single window**: One window per application

## Status

Graphicility is currently under heavy development.
APIs may change as the library is refined through real-world use.

## Final Notes

Any Kind of Contributions are welcome! From Extending to Changing out the Core check out the [CONTRIBUTING.md](./CONTRIBUTING.md)
