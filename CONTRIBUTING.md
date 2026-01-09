# Contributing to Graphicility

Thank you for your interest in Graphicility!
This document explains **how to contribute**, and just as importantly, **where certain ideas belong**.

Graphicility is intentionally minimal. Not every good idea belongs in the core library and that’s by design.

---

## Two Ways to Build with Graphicility

There are **two different ways** to contribute to Graphicility. Choose the one that fits your goal.

---

## 1. Building _on top of_ Graphicility (Extensions)

If you want to:

- Add higher-level drawing utilities
- Build a UI layer or widget system
- Add animation helpers
- Experiment with alternative APIs
- Provide domain-specific tools (emulators, tooling, etc.)

**Do NOT submit a PR to Graphicility core.**

Instead:

- Use the **public API**
- Use the **Extensibility API** (when available)
- Create a **separate crate** that depends on `Graphicility`

### Why?

Graphicility aims to stay small and stable.
Extensions allow experimentation **without increasing core complexity**.

This approach:

- Keeps Graphicility focused
- Prevents breaking changes for downstream users
- Encourages innovation without fragmentation

> Extensions are strongly encouraged and may be linked from the README in the future.

---

## 2. Contributing to Graphicility Core (Internal Changes)

If you want to:

- Improve performance
- Fix bugs
- Improve documentation
- Refine the public API
- Improve backends or internals
- Add *core* primitives aligned with the project philosophy

👉 **Fork the repository and submit a Pull Request.**

### Guidelines for Core Contributions

- Changes must align with Graphicility’s philosophy:
  - Minimal
  - Explicit
  - Immediate-mode
- Public API changes must be justified and documented

---

## Stability & Internal APIs

Only the following are considered stable:

- Public APIs documented in the README and Docs.rs

Everything else:

- May change without notice
- Should not be relied upon by external crates
- Is free to be refactored as needed

If you depend on internal behavior, you should:

- Fork Graphicility

---

## What Graphicility Will Not Accept

To avoid confusion, the following will **not** be accepted into core:

- UI widgets or layout systems
- Game engine features(Like Physics systems, Scenes Systems etc.)
- Asset pipelines
- Audio systems
- Retained-mode rendering
- ECS or scene graphs

These are better implemented **on top of Graphicility**, not inside it.

---

## Code Style & Quality

- Follow `rustfmt`
- Avoid unnecessary dependencies
- Document public APIs
- Add examples where appropriate

---

## Questions & Discussion

If you’re unsure where your idea fits:

- Open a discussion or issue
- We’re happy to help guide you in the right direction

---



