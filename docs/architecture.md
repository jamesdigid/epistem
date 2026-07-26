# Architecture

Epistem is organized around capabilities, not packages.

The domain shape is:

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

The layers are:

Capability

↓

Catalog

↓

Registry

Catalog is the reasoning surface for lookup, semantic discovery, and dependency
traversal. Registry handles persistence, transport, and synchronization.

The filesystem only stores capabilities. A directory is just one way to load a
capability source; it is not the domain object itself.

## Modules

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
