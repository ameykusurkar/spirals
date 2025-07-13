# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Native Build and Run
- `cargo run --release --features native` - Generate prime spiral image (creates test.png)
- `cargo build --release --features native` - Build optimized native binary
- `cargo build --features native` - Build debug native binary

### WebAssembly Build
- `wasm-pack build --target web --no-default-features --features wasm` - Build WebAssembly module
- `python3 web/serve.py` - Start development server for WebAssembly app (http://localhost:8000/web/)

### Testing and Quality
- `cargo test --features native` - Run unit tests for prime detection
- `cargo fmt` - Format code according to Rust standards
- `cargo fmt --all -- --check` - Check formatting (used in CI)

## Project Architecture

This is a Rust project that generates artistic visualizations of prime numbers arranged in a spiral pattern. It supports both native execution (saves PNG files) and WebAssembly execution (renders to HTML canvas).

### Core Components
- **PixelBuffer** (lib.rs): Raw RGBA pixel manipulation for WebAssembly compatibility
- **Buffer** (main.rs): Native image wrapper around `DynamicImage` for file output
- **PointGenerator**: Generates spiral coordinates using polar math (radius and theta increment)  
- **is_prime()**: Optimized prime detection function with comprehensive test coverage
- **generate_spiral_data()**: Core algorithm that can be called from both native and WebAssembly contexts

### Dual Build System
- **Native mode** (`--features native`): Traditional binary that saves PNG files using the `image` crate
- **WebAssembly mode** (`--features wasm`): Compiles to WASM for browser execution with HTML canvas rendering
- Feature flags control which dependencies and code paths are used

### Frontend (WebAssembly)
- **HTML/CSS/JS** in `web/` directory provides interactive canvas interface
- **Parameter controls** for width, height, and number of points
- **Real-time rendering** with performance monitoring
- **Image download** functionality for generated spirals

### Key Behavior
- Generates spiral coordinates and tests each position's index for primality
- Only prime-indexed positions get plotted as colored pixels
- Uses scaling to fit large coordinate ranges into image dimensions
- WebAssembly version provides browser console performance logging

### Dependencies
- `image` crate (0.23.12) for PNG generation (native only)
- `wasm-bindgen`, `web-sys`, `js-sys` for WebAssembly browser integration