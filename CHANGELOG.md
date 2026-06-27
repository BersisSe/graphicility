# Graphicility Changelogs
All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0]
### Added
- New `Window` struct for full control over the frame loop.
- `Window::next_frame()` — blocks until the next frame is ready, ideal for emulators and simulators.
- `Window::begin_frame()` — low-level frame start, returns `Option<&mut FrameContext>` for power users.
- `Window::end_frame()` — presents the current frame to the screen.
- `Window::is_running()` — returns whether the window is still open.
- New `window` example demonstrating the `Window` API.

### Changed
- `Runtime` is no longer generic over the draw function — it is now a pure state machine.
- `run()` and `run_with()` are now thin wrappers over `Window`, behavior is unchanged.

## [0.4.0]
### Added
- New `letterboxing` field to Config.
- New `use_letterboxing` method in the ConfigBuilder.
- New `Backend` trait for supporting more than one backend in the future
### Changed
- The letterboxing behavior observed in the renderer can now be toggled via config. Thanks to [PR from "CheeseBugles"](https://github.com/BersisSe/graphicility/pull/1)

## [0.3.0] - 2026-01-22
### Added
- `Rect` type for typed shape geometry and simpler bounds checks.
- `Extension` trait to implement custom extensions that connect into the application lifecycle.
- Config now has an `extensions` field for registering extensions.
- Finalized the **Extensibility API**.
- New `extension` feature to enable the **Extensibility API**.
- `Into<(u32,u32)>` trait is now implemented for `Vec2`.

**Note**: *See the new [Developing Extensions](./DEVELOPING_EXTENSIONS.md) page for more info.*

### Changed
- Bouncing Rect example now uses the builder instead of manual Config generation.

### Fixed
- `Input` not detecting `mouse_pressed` events correctly.

## [0.2.0] - 2026-01-07
### Added
- New Input system using the `Input` struct.
- New `Vec2` struct for typed geometry.
- New `FrameContext` for splitting logic between Graphics and Input.
- New `circle` drawable in `Graphics`.
- Target FPS selection via `Config`.
- Many more examples.

### Changed
- `run` and `run_with` now take a `FrameContext` closure instead of `Graphics` — **breaking change**.
- Defaults for `Config` logical and physical sizes changed from 800×600 to 1280×800 physical and 640×400 logical.
- All `Graphics` drawing functions now accept `impl Into<Vec2>`.
- Most internal doc comments refactored.

## [0.1.0] - 2026-01-01
Initial Release.