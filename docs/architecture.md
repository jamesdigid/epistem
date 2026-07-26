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

`src/verification` is reserved for trust, verification, outcome validation, and
human steering primitives.
