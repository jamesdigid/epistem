# Epistem

Epistem is an open capability registry for autonomous agents.
Traditional package managers distribute executable code. Epistem distributes reusable
capability definitions that autonomous agents can discover, compose, verify, and
satisfy through interchangeable providers.
It is written in Rust and is intentionally **not** an agent runtime, an LLM wrapper,
or a networking stack.

The first milestone is infrastructure only: manifest parsing, validation, filesystem
loading, capability resolution, and clean interfaces for the catalog, registry, and
search layers.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/jamesdigid/epistem/main/install.sh | sh
```

The installer downloads a prebuilt binary for your platform (macOS and Linux,
`x86_64` and `aarch64`) into `~/.epistem/bin`. If no prebuilt binary is available
for your platform it falls back to building from source with `cargo`, which
requires a [Rust toolchain](https://rustup.rs).

Then, from inside your agent directory:

```bash
epistem learn browser-generic
```

To scaffold a new capability directory first:

```bash
epistem init
```

You can also point it at a directory explicitly:

```bash
epistem init <directory>
```

If you omit the directory, Epistem initializes the parent directory of the
current working directory. The scaffold creates `epistem.json`, `README.md`,
and a starter capability package under `capabilities/<name>/` with its own
`README.md` and `AGENT.md` if they do not already exist.

Useful environment variables:

- `EPISTEM_INSTALL_DIR` — install location (default `~/.epistem/bin`)
- `EPISTEM_VERSION` — pin a specific release tag (default `latest`)
- `EPISTEM_FROM_SOURCE=1` — force a source build with `cargo`

## What Exists Today

- `epistem.json` manifest models with `serde`
- JSON parsing and schema validation
- capability loading from disk
- dependency graph construction hidden behind traits
- registry, catalog, and search interfaces with local stubs
- a `clap` CLI with a working `validate` command
- unit tests for the core scaffolding

## Architecture

Epistem is capability-first.

The domain model is easiest to read as a tree:

```text
Capability
│
├── Metadata
├── Contracts
├── Providers
├── Artifacts
└── Dependencies
```

Capability is the aggregate root.
Contract defines the interface and guarantees.
Providers are implementations.
Dependencies express capability requirements.
Artifacts are everything needed to understand or execute the provider.

A higher-order capability composes other capabilities into a new reusable abstraction
with its own contract surface.

Contracts are surfaced through the catalog and registry layers.

The filesystem only stores capabilities. A directory is just one way to load a
capability source; it is not the domain object itself.

### `src/manifest`
Owns the manifest schema, parsing, and validation rules for capabilities and the
contracts they satisfy.

### `src/storage`
Filesystem adapter that loads capabilities from disk.

### `src/catalog`
The reasoning surface for lookup, semantic discovery, and dependency traversal.
It composes registry, search, and resolver capabilities.

### `src/registry`
Defines infrastructure for persistence, transport, and synchronization of capabilities.
`LocalRegistry` is the current local implementation, while `RemoteRegistry` is a stub
for later work.

### `src/search`
Defines the catalog search interface. `LocalSearch` is a stub until semantic search exists.

### `src/resolver`
Builds dependency graphs from contracts behind traits.

### `src/cli`
Provides the command line surface. `validate` is implemented now; the rest are
placeholders so the interface is stable early.

### `src/models`
Contains small shared domain primitives.

### `src/error`
Centralizes application errors so adapters can translate failures cleanly.

### `src/verification`
Reserved for trust, verification, outcome validation, and human steering primitives.

### `src/utils`
Holds filesystem helpers and other reusable utilities.

## Example Manifest

```json
{
  "name": "@epistem/gmail-send",
  "version": "1.0.0",
  "description": "Send email using Gmail",
  "contracts": [
    {
      "id": "send-email",
      "version": "^1.0"
    }
  ],
  "dependencies": [
    {
      "contract": "authenticate-google"
    }
  ],
  "keywords": ["gmail", "email", "notification"],
  "artifacts": {
    "guide": "README.md",
    "examples": "examples/",
    "tests": "tests/"
  }
}
```

The current implementation still uses a fixed artifact shape. The longer-term
direction is to model artifacts as a list of typed entries so the manifest can
cover guides, instructions, schemas, workflows, embeddings, and other outputs
without hardcoding field names.

## Validate a Capability

```bash
cargo run -- validate examples/gmail-send
```

This prints a table with the manifest fields and validation status.

## Development

```bash
cargo test
cargo fmt
cargo clippy
```

Requires Rust 1.85 or newer.

## Roadmap

The next milestones can add capability installation, registry synchronization,
semantic search, and richer dependency resolution without changing the core
module boundaries.
