# Crate And Module Responsibilities

This document is the source of truth for entry-point and crate ownership
boundaries. Keep entries short enough to remain useful during review.

## Entry Points

- `bin/game` is the production desktop composition root. Keep it limited
  to Bevy setup, window configuration, top-level plugin registration,
  and high-level app state wiring.
- `bin/devtools` is the development tooling composition root. Use it for
  dev-only workspace plugins, previews, debug tools, and editor-like
  flows that should not ship in the production entry point.
- `bin/mobile` is the mobile composition root. Keep platform-specific
  Bevy setup and top-level plugin wiring here, and move shared behavior
  into crates.

## Crates

- `crates/core` owns shared app-level contracts that are not narrower
  subsystem concerns, such as common states, scheduling contracts,
  domain primitives, and cross-crate plugin groups as they are
  introduced. It should not own presentation, device input, loading
  implementation, or feature-specific gameplay rules.
- `crates/conf` owns configuration defaults, loading, validation, and
  runtime profiles as they are introduced. It should expose typed
  configuration data instead of letting other crates parse raw
  environment, file, or command-line data.
- `crates/camera` owns camera setup, camera behavior, and camera-facing
  visualization as they are introduced. It may consume state or loading
  events from other crates, but must not own generation rules, raw device
  input, or UI flow.
- `crates/gui` owns player-facing HUD, menus, panels, and editor UI
  modules as they are introduced. UI should translate interaction into
  explicit events or commands rather than owning simulation state.

## Maintenance

- When adding, deleting, renaming, or changing a module under `crates`,
  update this document in the same change with the module's role,
  ownership boundary, and important exclusions.
- When adding, deleting, renaming, or changing an entry point under
  `bin`, update this document in the same change with its composition
  responsibility and non-goals.
