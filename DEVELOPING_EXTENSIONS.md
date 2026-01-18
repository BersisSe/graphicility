# Developing Extensions for Graphicility

**Written for Graphicility 0.3.0 with Extensibility API Version 1**

Graphicility extensions are similar to middlewares for your graphics loop. They allow you to hook into the application lifecycle to manage global state, automate rendering tasks, or provide high-level APIs to the user.

## The Extension Trait

To create an extension, implement the `Extension` trait:

```rust
pub trait Extension {
    /// Called once when the Runtime is initialized.
    /// Ideal for loading fonts, textures, or setting up data structures.
    fn on_init(&mut self) {}

    /// Called every frame BEFORE the user's draw function.
    /// Ideal for layout calculations or clearing specific buffers.
    fn pre_draw(&mut self, ctx: &mut FrameContext) {}

    /// Called every frame AFTER the user's draw function.
    /// Ideal for UI overlays, debug consoles, or post-processing.
    fn post_draw(&mut self, ctx: &mut FrameContext) {}
}

```

---

## Best Practices & Patterns

### 1. Extending `FrameContext` (The Trait Extension Pattern)

If your extension provides new functionality (like a UI library), don't force the user to interact with your extension object directly. Instead, extend the `FrameContext` using a trait. This makes your library feel like a native part of Graphicility.

```rust
pub trait MyUiExt {
    fn button(&mut self, text: &str) -> bool;
}

impl MyUiExt for FrameContext<'_> {
    fn button(&mut self, text: &str) -> bool {
        // Use self.graphics() and self.input() to build your widget
        // Return true if clicked
    }
}
// In the main loop the end user can simply call ctx.button()
fn main(){
    graphicility::run(|ctx|{
        if ctx.button("Click Me!"){
            println!("You clicked!")
        }
    })
}
```
**Note**: This is a native feature of Rust's trait system and works even if the `extension` feature is disabled in the core. It allows your library to "add" methods to the Graphicility context.

### 2. Managing State (The Handle Pattern)

Extensions are **owned by the runtime**, not by the user’s draw loop.
Once registered, the extension instance itself is no longer directly accessible.

If your extension needs to expose **mutable state or controls to the user at runtime** (for example: toggling visibility, changing a theme, or resetting counters), use a **shared handle**.

The extension owns the state.
The user holds a cloneable handle to it.


### Basic Pattern

Use interior mutability to share state safely between:

- the extension lifecycle (`pre_draw`, `post_draw`)
- the user’s draw loop

For single-threaded contexts, prefer `Rc<RefCell<T>>`.
For multi-threaded contexts, use `Arc<Mutex<T>>`.

```rust
use std::rc::Rc;
use std::cell::RefCell;

pub struct Settings {
    pub visible: bool,
}

pub struct MyExtension {
    settings: Rc<RefCell<Settings>>,
}

#[derive(Clone)]
pub struct MyExtensionHandle {
    settings: Rc<RefCell<Settings>>,
}
```

### Constructor + Handle Exposure

The extension creates the shared state and exposes a **handle** to the user.

```rust
impl MyExtension {
    pub fn new() -> (Self, MyExtensionHandle) {
        let settings = Rc::new(RefCell::new(Settings {
            visible: true,
        }));

        (
            Self {
                settings: settings.clone(),
            },
            MyExtensionHandle { settings },
        )
    }
    // Or just expose a handle a method
    pub fn handle(&self) -> MyExtensionHandle{
        MyExtensionHandle {
            settings: self.settings.clone(),
        }
    }
}
```

### Reading State Inside the Extension

```rust
impl Extension for MyExtension {
    fn post_draw(&mut self, ctx: &mut FrameContext) {
        if !self.settings.borrow().visible {
            return;
        }

        // draw overlay
    }
}
```



### When You *Don’t* Need a Handle

You *do not* need shared state if:

- the extension is fully autonomous
- configuration never changes after initialization
- interaction only happens through `FrameContext` trait extensions

In those cases, store state directly inside the extension.



### 3. Correct Constructors
Make sure your extension has a simple to use constructor.

- new(): Use this for simple extensions with few configuration points.
     - tuple(self,handle): If your Extension returns a handle using a tuple might be good idea!
- Builder Pattern: Use this for complex extensions (like a UI system) that have many settings or optional data points.
    - For builders we recommend the use of a `handle()` method.


### 4. Coordinate System
Its important to use the correct Coordinate system to use for your extension.
- Logical Size: This is the virtual resolution (the canvas size). You should use this 99% of the time for drawing to ensure your extension scales correctly on all screens.

- Window Size: The physical window resolution. Only use this for window-level calculations (like calculating where to place a window-border overlay).

---

## Example: A Simple Grid Overlay

This extension draws a coordinate grid *underneath* the user's drawings.

```rust
use graphicility::extensions::Extension;
use graphicility::{Color, FrameContext};

pub struct GridExtension {
    pub spacing: u32,
}

impl Extension for GridExtension {
    fn pre_draw(&mut self, ctx: &mut FrameContext) {
        let (w, h) = ctx.graphics().logical_size();
        for x in (0..w).step_by(self.spacing as usize) {
            ctx.graphics().line((x, 0), (x, h), Color::rgb(50, 50, 50));
        }
        for y in (0..h).step_by(self.spacing as usize) {
            ctx.graphics().line((0, y), (w, y), Color::rgb(50, 50, 50));
        }
    }
}

```

## Registration

Users register your extension during the configuration phase:

```rust
let config = Config::builder()
    .with_extension(MyExtension::new())
    .build();
// Or
let ext = MyExtension::new()
let config = Config::builder()
    .with_extension(ext)
    .build();
// If its a stateful extension
let (ext,handle) = MyExtension::new()
let config = Config::builder()
    .with_extension(ext)
    .build();


```

