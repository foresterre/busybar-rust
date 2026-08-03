# Changelog

All notable changes to this crate are documented in this file. The format is based on Keep a Changelog.

## Unreleased

### Added

- Added `busybar capture-frames <dir>` subcommand which writes frames streamed by a screen to a folder.
- Added `--timeout` option, which sets the API timeout for the subcommands which do not specify one themselves.

### Removed

- Removed `--frame-dir` of `busybar api streaming status-ws` (replaced by `busybar capture-frames`), so `status-ws` follows the API call more closely.

## 0.0.10 - 2026-08-03

### Added

- Added `busybar mirror` subcommand which mirrors and renders the front device screen in the terminal.

### Changed

- Moved the operations which map directly onto the HTTP API under a `busybar api` subcommand.

## 0.0.9 - 2026-08-03

### Added

- Implemented `busybar streaming status-ws` which streams device status over a WebSocket.

## 0.0.8 - 2026-08-02

### Added

- Frames of the front and back screen can now be re-encoded to `bmp`, `jpg` or `png`.

## 0.0.7 - 2026-08-02

No notable changes.

## 0.0.6 - 2026-08-02

No notable changes.

## 0.0.5 - 2026-08-02

### Added

- Add stubs for subcommands for most sections of the API.
- Implemented the subcommands for the getters of the `system` section.

### Changed

- The subcommands now follow the API spec more closely.

## 0.0.4 - 2026-08-02

### Added

- Implemented `busybar wifi status`.

## 0.0.3 - 2026-08-02

### Maintenance

- Include readme in published crate.

## 0.0.2 - 2026-08-02

### Changed

- Model names used in reported events now follow the original API docs more closely.

## 0.0.1 - 2026-08-02

### Added

- Added first CLI for the BUSY Bar HTTP API.
