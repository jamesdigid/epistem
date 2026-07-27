# Epistem

Epistem is an open capability registry for autonomous agents.

It helps agents discover, acquire, initialize, and verify capabilities through interchangeable providers.

## Quick Start

```bash
curl -fsSL https://raw.githubusercontent.com/jamesdigid/epistem/main/install.sh | sh
```

```bash
epistem init
epistem learn browser-attach
```

`epistem init` creates a workspace manifest (`epistem.yaml`) plus a `capabilities/` directory.
`epistem learn` looks up a capability in the registry, resolves a provider, acquires it, initializes it, and runs verification tests before marking it installed.

## Provider Layout

Existing projects become Epistem-compatible by adding a lightweight `capability.yaml` manifest.

```text
my-project/
├── capability.yaml
├── prompt.md
├── tests/
└── existing project files...
```



## Current Phase

Phase 1 focuses on bootstrap infrastructure:

- capability discovery
- provider lookup
- installation and acquisition
- runtime initialization
- verification tests
- workspace recording
- release and install smoke checks



## Development

```bash
cargo test
cargo fmt
cargo clippy
```

Requires Rust 1.85+.

## Roadmap

- GitHub-backed provider discovery
- richer provider selection
- semantic search
- remote registries
- broader capability sets such as `browser-generic`



## License

Apache License 2.0