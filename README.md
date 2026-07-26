# Epistem

Epistem is an open capability registry for autonomous agents.

Instead of distributing executable code, Epistem distributes reusable capabilities that agents can discover, compose, verify, and satisfy through interchangeable providers.

It is written in Rust and intentionally focuses on capability management—not agent runtimes, LLM orchestration, or workflow execution.

---

## Getting Started

### 1. Install Epistem

```bash
curl -fsSL https://raw.githubusercontent.com/jamesdigid/epistem/main/install.sh | sh
```

### 2. Create an agent workspace

```bash
mkdir amy
cd amy

epistem init
```

This creates a new capability workspace.

### 3. Teach your agent something new

```bash
epistem learn browser-generics
```

Epistem resolves dependencies, downloads the required artifacts, and installs the capability into your agent workspace.

---



## Project Structure

After running `epistem init`:

```text
amy/
├── epistem.json
├── capabilities/
└── EPISTEM.md
```

---



## Current Status

The project is in its early infrastructure phase.

Current functionality includes:

- Capability manifest parsing and validation
- Local capability loading
- Dependency graph construction
- Registry, catalog, and search abstractions
- CLI scaffolding
- Unit tests for core components

---

## Development

```bash
cargo test
cargo fmt
cargo clippy
```

Requires Rust 1.85+.

---



## Roadmap

Near-term milestones include:

- Build out a browser generic capability
- Work out the agent integration schema
- Runtime integration
- Capability installation
- Semantic search
- Dependency resolution



## License

Apache License 2.0