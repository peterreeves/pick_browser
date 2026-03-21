# Pick Browser

A browser picker. When you (or another application) open a link anywhere on your system, Pick Browser intercepts it and lets you choose which browser to open it in.

## Installation

### Prerequisites

- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

### Build from source

```sh
npm install
npm run tauri build
```

The installer will be output to `src-tauri/target/release/bundle/`.

### Development

```sh
npm install
npm run tauri dev
```

## Usage

1. Run the build step, and run the installer that was created.
2. Set Pick Browser as your default browser (there's a button in the app, or do it manually via your OS settings).
3. Click any link — Pick Browser will pop up and let you choose which browser to open it in.
