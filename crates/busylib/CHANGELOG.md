# Changelog

All notable changes to this crate are documented in this file. The format is based on Keep a Changelog.

## Unreleased

### Added

- Added a `weather` example, which draws the current Amsterdam weather.

## 0.0.11 - 2026-08-03

No notable changes.

## 0.0.10 - 2026-08-03

No notable changes.

## 0.0.9 - 2026-08-03

### Added

- Implemented `streaming status_ws` which streams device status over a WebSocket.
- Added WebSocket traits and included tungstenite based WebSocket transport.

## 0.0.8 - 2026-08-02

No notable changes.

## 0.0.7 - 2026-08-02

No notable changes.

## 0.0.6 - 2026-08-02

### Added

- Documentation for the `Client`.

## 0.0.5 - 2026-08-02

### Added

- Implemented all operations of the `system` section of the API.
- Added `streaming` API module, excluding the `status_ws` which requires websocket streaming.

### Changed

- The API modules follow the API spec more closely, and the `audio` and `display` models can now be found under `assets`.

## 0.0.4 - 2026-08-02

### Fixed

- Fix issue where the API prefix of the local device differed from the one in the OpenAPI spec, which made the API inaccessible.

## 0.0.3 - 2026-08-02

No notable canges.

## 0.0.2 - 2026-08-02

### Added

- Added documentation and examples for the API methods, endpoints and models, based on the OpenAPI spec.

### Changed

- Model names follow the original API docs more closely.
- Types moved into their own modules.

## 0.0.1 - 2026-08-02

### Added

- Added a transport independent HTTP client for the BUSY Bar API.
