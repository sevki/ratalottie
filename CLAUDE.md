# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RataLottie is a Rust library that provides a ratatui widget for displaying Lottie animations in terminal user interfaces. It implements the ratatui Widget trait and uses dotlottie-rs for animation rendering.

## Development Commands

### Build Commands
- `cargo build` - Build the library
- `cargo build --release` - Build optimized release version
- `cargo check` - Check code without building

### Testing
- `cargo test` - Run all tests
- `cargo test --lib` - Run library tests only
- `cargo run --example <example_name>` - Run specific visual example
- `cargo run --example all` - Run all visual examples (if implemented)

### Visual Testing
Since this is a visual widget library, create examples that demonstrate:
1. Basic animation rendering
2. Different animation sizes and positions
3. Animation controls (play, pause, loop)
4. Multiple animations on screen
5. Animation performance in terminal

Example structure:
```
examples/
├── basic.rs          # Single animation demo
├── multiple.rs       # Multiple animations
├── controls.rs       # Interactive controls
└── performance.rs    # Performance testing
```

Run examples with: `cargo run --example basic`

### Code Quality
- `cargo fmt` - Format code
- `cargo clippy` - Run linter

### Documentation
- `cargo doc --open` - Generate and open documentation

### Dependencies
- `cargo add <package>` - Add new dependencies (always use this instead of manually editing Cargo.toml)

## Architecture

The library is structured as a ratatui widget that:
1. Wraps dotlottie-rs functionality for loading and rendering Lottie animations
2. Implements the ratatui `Widget` trait to integrate with terminal UI applications
3. Should reference the [dotlottie-rs demo-player](https://github.com/LottieFiles/dotlottie-rs/blob/main/examples/demo-player/src/main.rs) for implementation patterns

## Key Dependencies

- **ratatui** (v0.29.0) - Terminal UI framework
- **dotlottie-rs** (v0.1.0-alpha.1) - Lottie animation library
- **ratatui-widgets** (v0.2.2) - Additional widget utilities

## Development Notes

- The project uses Rust edition 2024
- Create bindings for the original dotlottie-rs functionality rather than copying code
- The main entry point is `src/lib.rs`
- Visual examples are essential for testing animation rendering in the terminal