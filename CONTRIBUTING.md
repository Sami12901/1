# Contributing to CapBD

Thank you for your interest in contributing to **CapBD: Photo & Video Editor in Rust**!

## Architecture Overview

CapBD is built as a clean, modular Cargo workspace:
- **`crates/capbd-core`**: Pure Rust data structures (Timeline, Tracks, Clips, Timecodes, Undo/Redo, Project JSON format). No UI or GPU dependencies.
- **`crates/capbd-engine`**: Media processing, frame decoders, WGSL shaders, audio mixer, and export compositor.
- **`crates/capbd-ui`**: CapCut-style dark UI components, vector icons, magnetic timeline widget, viewport, and inspector.
- **`crates/capbd-desktop`**: Desktop runner for Windows, Linux, and macOS using `eframe` with `wgpu`.
- **`crates/capbd-android`**: Android NDK native activity runner.

## Development Workflow

### Prerequisites
- Rust 1.80+ (`rustup update stable`)
- Git

### Build & Run Locally
```bash
# Clone the repository
git clone https://github.com/capbd/capbd.git
cd capbd

# Run all unit tests
cargo test --workspace

# Launch the desktop editor on Windows / Linux / macOS
cargo run -p capbd-desktop
```

### Linux Build Dependencies
On Debian/Ubuntu:
```bash
sudo apt-get install libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev libvulkan-dev pkg-config
```

### Code Style
- Run `cargo fmt --all` before committing.
- Run `cargo clippy --workspace --all-targets` to ensure no lints or warnings.
- Keep UI icons strictly vector-drawn (no emojis).
