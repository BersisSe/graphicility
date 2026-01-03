# Graphiclity Roadmap

This roadmap outlines planned improvements while keeping the core small, explicit, and approachable.
✅ = Means Complete
---

## Near Term

### Documentation
- Improve README clarity and structure
- Document public APIs and design decisions

---

### Internal Polish
- Clean up internal rendering and command flow
- Improve naming and API consistency
- Reduce internal complexity where possible
- Stabilize existing public APIs

---

### Input System
- Solid, frame-based input handling

---

## Mid Term

### Extensibility API
- Introduce a **minimal and optional** extensibility layer
- Expose a stable drawing command boundary
- Allow external libraries hook up to the internals
- Keep `run` / `run_with` simple and unchanged

This API is intended for Building on top Graphiclity. Eg. Widget Systems, UI Frameworks, Layout Engines

---

## Long Term

### Experimental Layers (Out of Core)
- UI or tooling experiments built *on top* of Graphiclity
- No widgets, layout systems, or retained state in core
- Core remains renderer-agnostic and immediate-mode

---
