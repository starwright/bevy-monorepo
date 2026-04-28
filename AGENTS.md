# AGENTS.md

## Scope

This repository hosts a new game project built with Rust and Bevy. Keep
design and implementation decisions aligned with long-term clarity,
modularity, and incremental delivery.

## Product Philosophy

- Favor strategic depth over mechanical speed. Prefer planning,
  tradeoffs, spatial control, and timing over click pressure or input
  spam.
- Build systemic gameplay. Economy, movement, combat, progression, and
  map control should interact through clear rules.
- Prefer originality over imitation. Borrow genre conventions when
  useful, but do not default to clone design.
- Reward sustained good decisions more than hidden bonuses, bursty
  randomness, or short-lived spikes.
- Use modern hardware deliberately. Scale and simulation are valuable
  only when they improve gameplay and remain understandable.
- Ship incrementally. Keep changes atomic, reviewable, and compatible
  with a stable main branch.

## Requirement Intake And Decomposition

- Understand the player-facing or developer-facing requirement before
  editing code. Identify the goal, constraints, affected systems, and
  non-goals when they are clear from context.
- If the current request lacks enough context, first read the relevant
  game design documents under `docs` and the affected current codebase
  modules before choosing an implementation approach. If the existing
  conversation and local context are already sufficient, do not expand
  discovery work unnecessarily.
- Split each concrete requirement into small implementation slices with
  an explicit owner crate or module, owned data, public boundary, and
  validation plan.
- Prefer mapping slices to existing crates and modules before creating a
  new top-level subsystem. Create a new crate or module only when the
  existing owner would lose a clear single responsibility.
- Keep slices separated by responsibility. Do not mix simulation,
  rendering, input, UI, persistence, configuration, and loading changes
  in one module just because they serve one feature.
- For Bevy work, identify the owning plugin, app state, schedule,
  systems, events, resources, and components for each slice before
  implementation.
- If a requirement is too broad for one safe change, implement the
  smallest coherent slice and document the remaining design decisions in
  the appropriate docs or follow-up issue.

## Core Engineering Principles

- Keep the composition root thin. Entry points should mostly initialize
  Bevy, register plugins, and wire top-level states.
- Break functionality into small, self-contained modules or crates with
  narrow public APIs.
- Prefer clear ownership. A subsystem should own its own data,
  invariants, and mutation paths.
- Prefer explicit events or messages for communication between
  subsystems.
- Avoid tight coupling, hidden ordering requirements, and cross-module
  reach-through.
- Favor safe and simple Rust. Avoid `unsafe`, over-abstraction, and
  clever indirection unless there is a concrete payoff.
- Default to private visibility and widen only when necessary.
- Maintain the single responsibility principle at both crate and module
  boundaries. A module should have one primary reason to change; split it
  when it starts owning unrelated data, lifecycle, or policy.

## Bevy Technical Constraints

- Each major subsystem should have a clear Bevy plugin boundary.
- If the repository uses multiple crates, each Bevy-oriented crate
  should expose a top-level `Plugin` or `PluginGroup` from `lib.rs`.
- Keep crate `lib.rs` files as thin boundaries for module declarations,
  public re-exports, and `*PluginGroup` definitions and implementations.
  Do not put systems, components, resources, events, helper functions, or
  individual feature `Plugin` implementations in `lib.rs`; place actual
  feature code in dedicated module files.
- Keep systems, events, resources, components, and helpers close to the
  plugin that owns them.
- Prefer multiple focused plugins over one large plugin. Split by
  responsibility, not by arbitrary file count.
- Use explicit app states for major phases such as boot, loading, menu,
  gameplay, pause, and shutdown. Prefer state-driven flow over boolean
  flag orchestration.
- Use `OnEnter`, `OnExit`, and state-scoped schedules for setup and
  teardown. Do not hide lifecycle work in always-on update systems.
- Keep systems small and parameter lists manageable. When a system grows
  too wide, split it and connect the pieces with events or owned
  resources.
- Do not make cross-plugin assumptions about event timing, uniqueness,
  or absence of duplicates.
- Do not rely on implicit scheduling order. When order matters, express
  it with sets and targeted ordering constraints.
- Keep ordering constraints local and minimal. If everything needs
  `.before()` and `.after()`, the subsystem boundaries are wrong.
- Prefer event-driven or change-driven work over polling every frame.
  Use `run_if`, state filters, and change detection to keep hot paths
  lean.
- Put deterministic or simulation-critical logic in a fixed-timestep
  schedule when appropriate. Do not make gameplay outcomes depend on
  frame rate.
- Keep rendering, input, simulation, audio, UI, and networking
  separated by clear plugin boundaries.
- Use resources only for true global state or coordination data.
  Prefer components for entity-owned state.
- Components and resources exposed as part of a subsystem contract
  should be inserted, removed, and mutated only by the owning
  subsystem.
- Clean up transient entities, observers, and resources on state exit or
  subsystem shutdown.
- Name plugins with the `*Plugin` suffix, system sets with the `*Set`
  suffix, and events with the `*Event` suffix.

## Crate And Module Responsibilities

- Entry-point and crate ownership boundaries are documented in
  `docs/src/crate-and-module-responsibilities.md`.
- When adding, deleting, renaming, or changing a module under `crates`,
  update `docs/src/crate-and-module-responsibilities.md` in the same
  change with its role, ownership boundary, and important exclusions.

## Gameplay And Simulation Guidance

- New mechanics should strengthen the core loop and the game's identity.
- Prefer systemic interactions over one-off scripted exceptions.
- Keep player-facing rules legible. Complexity is acceptable; opacity is
  not.
- Avoid features that mainly add busywork, APM tax, or UI friction.
- Model scarcity, space, and timing explicitly when they matter to
  strategy.
- Keep simulation, presentation, input, and networking decoupled enough
  to evolve independently.

## Repository Guidance

- Keep runtime code, assets, documentation, and developer tooling
  clearly separated.
- If the project uses a workspace, keep each crate small, focused, and
  independently understandable.
- Prefer extending an existing domain module before adding a new
  top-level subsystem.
- Update documentation when changing player-facing rules, workflows,
  protocols, save formats, or configuration behavior.
- Store large binary assets with Git LFS or an equivalent large-file
  workflow.

## Validation

- Match the repository's CI configuration, lint rules, and formatter
  settings.
- Run the narrowest useful checks during iteration, then run the full
  relevant build, test, lint, and format steps before handoff.
- Add or update tests when changing domain rules, serialization,
  scheduling assumptions, or protocol boundaries.
- Treat warnings as debt. Do not leave new warnings behind unless the
  task explicitly requires it.

## Style Notes

- Let repository configuration files be the source of truth for
  formatting and linting.
- Keep comments brief and useful. Explain intent, invariants, schedule
  assumptions, or tricky behavior, not obvious line-by-line mechanics.
- Match existing naming, module placement, and plugin organization
  before introducing a new pattern.
