# Graphiclity Roadmap

This roadmap outlines planned improvements while keeping the core small, explicit, and approachable.
✅ = Means Complete
---

## Near Term

### Documentation
- Improve README clarity and structure
- Document public APIs and design decisions

---

### Internal Polish ✅
- Clean up internal rendering and command flow
- Reduce internal complexity where possible

---

### Input System ✅
- Solid, frame-based input handling
- Mouse and Keyboard inputs

---

## Mid Term

### Extensibility API
- Introduce a **minimal and optional** extensibility layer
- Expose a stable drawing command boundary
- Allow external libraries hook up to the internals
- Keep `run` / `run_with` simple and unchanged

This API is intended for Building on top Graphiclity. Eg. UI Frameworks, Game Frameworks, Layout Engines, Widget Systems,

### Better Text System
- Support for .tff fonts 
- Add a simple Text Renderer

### Wgpu Backend
- Pure Performant wgpu based backend for rendering
---
