# Endless Roll Frontend

Frontend for Endless Roll, a TTRPG campaign and game-session platform.

This project is currently a Rust/Yew web frontend compiled to WebAssembly with
Trunk. It includes early Tauri configuration for a desktop app, but the main
development path right now is the web app.

## Tech Stack

- Rust 2021
- Yew 0.23
- Yew Router
- Trunk
- WebAssembly
- Gloo HTTP utilities
- Optional Tauri scaffolding

## Prerequisites

Install these before running the app:

- Rust and Cargo: <https://rustup.rs>
- Node.js and npm: <https://nodejs.org>
- Trunk:

```bash
cargo install trunk
```

You also need the backend API running at:

```text
http://localhost:8000/api/v1
```

The frontend currently calls this API from the files in `src/api/`.

## Setup

You can use the setup script:

```bash
./setup.sh
```

Or install dependencies manually:

```bash
npm install
```

The project does not currently use npm packages for the app itself. `package.json`
mainly provides convenient scripts for Trunk.

## Development

Start the web development server:

```bash
npm run dev
```

Or with Make:

```bash
make dev
```

Trunk serves the app locally, usually at:

```text
http://localhost:1420
```

## Build

Build the web app for production:

```bash
npm run build
```

Or:

```bash
make build
```

The built files are written to:

```text
dist/
```

## Quality Checks

Check Rust compilation:

```bash
make check
```

Format Rust code:

```bash
make format
```

Run Clippy with warnings treated as errors:

```bash
make lint
```

## Project Structure

```text
.
├── src/
│   ├── api/              # Backend API clients
│   ├── components/       # Shared Yew components
│   ├── pages/            # Route-level pages
│   │   └── auth/         # Login and registration pages
│   ├── main.rs           # Yew app entry point
│   ├── models.rs         # Shared data models
│   ├── router.rs         # Application routes
│   └── store.rs          # App state placeholder
├── index.html            # Trunk HTML entry point
├── Cargo.toml            # Rust package and dependencies
├── Trunk.toml            # Trunk build config
├── package.json          # npm command wrappers
├── Makefile              # Common development commands
├── setup.sh              # Local setup helper
├── tauri.conf.json       # Tauri desktop config
└── build.rs              # Tauri build hook
```

## Routes

The app currently defines these routes:

- `/` - dashboard
- `/login` - login
- `/register` - registration
- `/campaigns` - campaigns page
- `/campaigns/:id` - campaign detail
- `/session/:session_id` - game session
- `/404` - not found

## Tauri Notes

This repository contains Tauri-related files:

- `tauri.conf.json`
- `build.rs`
- `tauri-build` in `Cargo.toml`
- `make desktop`
- `make build-desktop`

Use these only if you are actively building a desktop app. For normal web
frontend development, Trunk is enough.

Desktop commands:

```bash
cargo tauri dev
cargo tauri build
```

## Useful Commands

```bash
make help          # Show all Makefile commands
make setup         # Install project dependencies
make dev           # Start web dev server
make build         # Build web app
make check         # Check compilation
make format        # Format Rust code
make lint          # Run Clippy
make clean         # Remove build artifacts
```

## Backend Integration

The frontend expects authentication tokens to be stored in browser
`localStorage` under:

```text
auth_token
```

API modules currently cover:

- authentication
- campaigns
- characters
- NPCs
- sessions
- game-session WebSocket connection

The game session WebSocket currently connects to:

```text
ws://localhost:8000/api/v1/ws/game/{session_id}
```

