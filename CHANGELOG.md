# Changelog

## [Unreleased]

## [0.1.2](https://github.com/globe-and-citizen/layer8-primitives-rs/compare/v0.1.1...v0.1.2) - 2025-03-21

### Other

- revert ver bumps
- revert ver bumps
- ver bump
- with tests
- send actual json serialized types
- convenience derives
- make payload optional
- rm unrelated
- provide js feat for transient dep
- add metadata serde
- with docs & using bin for raw
- specify roundtrip format
- consistent naming
- use envelope
- websocket data wrapper
- pub envelope methods
- pub codec for envelope

### Changed

- Provided better transport types for Layer8Envelope. Includes `Layer8Envelope::Http`, `Layer8Envelope::WebSocket` and `Layer8Envelope::Raw`, the latter being a catch-all for any other transport type. [#7](https://github.com/globe-and-citizen/layer8-primitives-rs/pull/7)
- `RoundtripEnvelope` API are now public. [#6](https://github.com/globe-and-citizen/layer8-primitives-rs/pull/6)

### Added

- Provided a structure for Websocket messages, `WebSocketPayload`. Added tests for the transport formats. [#7](https://github.com/globe-and-citizen/layer8-primitives-rs/pull/7)
- Pinning the getrandom version and including the `js` feature for compatibility with WebAssembly target consumers. [#9](https://github.com/globe-and-citizen/layer8-primitives-rs/pull/9)

## [0.1.1](https://github.com/globe-and-citizen/layer8-primitives-rs/releases/tag/v0.1.1) - 2025-01-10

### Added

- Added an API for file compression using gzip encoded to/from a base64 string. [#1](https://github.com/globe-and-citizen/layer8-primitives-rs/pull/1)
