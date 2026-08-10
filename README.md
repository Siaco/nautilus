# nautilus
Nautilus is a high-performance, Rust-powered monorepo engineered to unify software deployment pipelines into a single ecosystem. From source acquisition to Kubernetes deploy, work seamlessly via Nautilus Deck—a keyboard-first TUI—or Nautilus Studio, a Tauri desktop GUI with sleek glassmorphism. Fast, graph-based, and built for control.

## 🧑‍💻 User Guide

### 1. Installation
Currently, Nautilus provides pre-built binaries via the GitHub Releases page:
- **Nautilus CLI:** A powerful terminal-based execution engine. Download the executable and add it to your system PATH.
- **Nautilus Studio:** A portable Desktop GUI (Tauri app). Download the zip, extract it, and run `nautilus-studio.exe` (Windows) or the equivalent on your OS.

*(Note: Binaries may trigger SmartScreen/Gatekeeper warnings during the early beta until we integrate EV Code Signing.)*

### 2. Getting Started
To get started with Nautilus, simply define your pipeline in a YAML format.

```yaml
version: "1.0"
pipeline:
  name: "Example Pipeline"
  stages:
    - id: "build"
      plugin: "shell"
      args:
        command: "cargo build --release"
```

Save this as `pipeline.yaml`, and run it using the CLI:
```bash
nautilus run pipeline.yaml
```

Alternatively, open Nautilus Studio, load your workspace, and trigger pipelines with a single click.

---

## 🛠️ Developer Guide

### 1. Architecture Overview
Nautilus is a Cargo Workspace containing several integrated components:
- `nautilus-core`: The headless DAG execution engine, state manager, and trait-based plugin system.
- `nautilus-cli`: The terminal user interface (TUI) and argument parser built on top of `clap`.
- `nautilus-studio`: The React/TypeScript frontend wrapped in a Tauri desktop shell.

### 2. Development Setup

#### Prerequisites
- Rust (`rustup` with the latest stable version).
- Node.js (v20+ for Nautilus Studio frontend).
- Platform-specific build tools (e.g. C++ build tools for Windows, `build-essential` & `libwebkit2gtk-4.1-dev` for Linux).

#### Building the Monorepo
You can build the entire Rust workspace at once:
```bash
cargo build
```

To run the unit and integration test suites:
```bash
cargo test
```

#### Running Nautilus Studio locally
```bash
cd crates/nautilus-studio
npm install
npm run tauri dev
```

### 3. Adding a Plugin
Plugins implement the asynchronous `Plugin` interface located in `nautilus-core`. Create a new struct under `crates/nautilus-core/src/plugin/builtins/`, implement `execute(&self, ctx: &ExecutionContext)`, and register it in the plugin registry!

---

## 📬 Let Us Know You're Using Nautilus!

Nautilus is open-source software released under the [Apache 2.0 License](./LICENSE). 

If you or your organization are using Nautilus in production, integrating it into your infrastructure, or building custom tooling on top of it, **we would love to hear from you!**

Reporting your usage helps us gauge adoption, prioritize core feature development, and support the project's long-term sustainability.

* **Open a Github Issue:** Drop a quick note in our [Adoption Registry](https://github.com/Siaco/nautilus/issues).
* **Direct Email:** Send a short note to `pastura.camillofrancesco@gmail.com`.