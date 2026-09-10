# Module Structure Audit

This audit establishes a repeatable baseline for the module-structure work in `REFACTORING_ROADMAP.md`.

## Scope

The audit covers Rust module roots under:

- `app/src`
- `crates`

The check intentionally does not prescribe a single architecture for UI, GUI, TUI, or terminal code. Those areas have different runtime constraints; the tool only identifies unusually deep module nesting for review.

## Automated check

Run:

```bash
./script/module-structure-lint
```

The default maximum depth is three path components below the scanned source root. Override it for exploratory audits:

```bash
MODULE_STRUCTURE_MAX_DEPTH=4 ./script/module-structure-lint
```

This is an audit tool, not an automatic move-and-rewrite tool. A reported module should be reviewed with its public exports, consumers, feature gates, tests, and platform-specific implementations before restructuring.

## Review checklist

For each reported module:

1. Identify the responsibility represented by the module and its children.
2. Record public modules and re-exports before changing paths.
3. Check all downstream imports with `rg` or repository search.
4. Preserve platform and feature-gated implementations.
5. Move tests with the behavior they cover.
6. Prefer private implementation modules and a small public facade.
7. Run formatting, targeted tests, and Clippy before opening a refactor PR.

## Current prioritization

The first review order follows the roadmap while keeping UI-heavy implementation changes separate from shared/core cleanup:

1. `crates/ai/` — mixed orchestration and execution responsibilities.
2. `crates/warp_core/` — shared APIs used across front ends; changes have broad impact.
3. `crates/warp_errors/` — candidate boundary for standardized error context.
4. `crates/warp_terminal/` — defer structural moves until the shared API audit is complete.
5. `crates/warp_tui/` and `crates/warpui_core/` — audit independently because they contain front-end-specific abstractions.

No module is moved solely because it is deep. A restructure should reduce coupling, clarify ownership, or shrink the public API.
