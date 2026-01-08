### Drawing
```rust
use graphiclity::{Color };

fn main() {
    /// Run is the main entry point of Graphiclity.
    /// It starts the Event loop and Initializes the Window.
    graphiclity::run(|ctx| {
        let g = ctx.graphics(); // We are only gonna draw so lets get the graphics handle
        g.clear(Color::WHITE) // Lets Clear our canvas at the start of the frame
        g.pixel((5,5), Color::RED); // Draw a single pixel in cords x:5 y:5
        // Draw a rectangle: (x, y), (width, height)
        g.rect((20, 20), (50, 50), Color::BLUE);
        // Draw some text too
        g.text((20, 100), "drawing text", Color::WHITE);
    });
}

```
### 2 Dimensional Vectors
```rust
use graphiclity::{Color, Vec2 };

fn main() {
    let point1 = Vec2::new(5,5); // Create a a new Vec2 using the `new` constructor.
    let point2 = Vec2{x: 100, y: 100}; // We can also construct it manually.

    graphiclity::run(|ctx| {
        let g = ctx.graphics(); // We are only gonna draw so lets get the graphics handle
        g.line(point1, point2, Color::RED); // Let's draw a line between these 2 points
        g.rect(point2, point2 + point1, Color::BLUE); // Let's draw a rectangle with the size of point1 + point2
    });
}

```
### Getting Inputs
```rust
use graphiclity::{Color,  KeyCode};

fn main() {
    graphiclity::run(|ctx| {
       let (g, input) = ctx.split(); // Split the context so we draw and read input without raging the barrow checker.
       g.clear(Color::WHITE);
       if input.key_pressed(KeyCode::Escape){ // Pressed only triggers once.
            println!("Key Espace is pressed");
       }
       if input.key_down(KeyCode::KeyW){ // Down triggers continuesly
            println!("Key w is down");
       }
       let mpos = input.mouse_pos(); // Returns the Mouse position
       if let Some(pos) = mpos{ // Always check if the mouse position is None it returns None if the window is out of focus.
           g.text(pos, format!("X:{}, Y:{}", pos.0,pos.1), Color::RED);
       }
    });
}

```


