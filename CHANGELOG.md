# Graphicility Changelogs

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## Added
- New `letterboxing` field to Config.
- New `use_letterboxing` method in the ConfigBuilder

## Changed
- The Letterboxing behavior observed in the renderer now can be toggled via config. Thanks to [PR from "CheeseBugles"](https://github.com/BersisSe/graphicility/pull/1)



## [0.3.0] - 2026-01-22
## Added
- `Rect` Type for typed shape geometry and simpler bounds checks.
- `Extension` Trait to implement custom Extensions that will connect into applications lifecycle.
- Config now has a `Extensions` field for registering Extensions
- Finalized the **Extensiblity API** 
- New `extension` feature to enable the **Extensiblity API**
- Into<(u32,u32)> trait is now implemented for `Vec2`

**Note** : *See the New [Developing Extensions](./DEVELOPING_EXTENSIONS.md) Page for more info*

## Changed 
- Bouncing Rect example now uses the Builder instead of manual Config generation.

## Fixed
- `Input` Not detecting mouse_pressed events correctly.

## [0.2.0] - 2026-01-07

### Added
- New Input System using the Input Struct.
- New Vec2 Struct for Typed Geometry.
- New `Window Context` for spliting logic between Graphics and Input.
- New Circle Drawable in `Graphics`.
- Target Fps selection via `Config`.
- A lot more examples!

### Changed
- `run` and `run_with` now take a FrameContext Closure instead of Graphics **this change is Breaking**.
- defaults for `Config` struct logical and physical sized changed from 800x600 to 1280x800 physical & 640&400 logical sizes
- All of `Graphics` drawing functions now accept `Into<Vec2>`
- Most of the Internal Doc Comments are Refactored

## [0.1.0] - 2026-01-01
Initial Release
