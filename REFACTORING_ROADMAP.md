# Warp Refactoring Roadmap: Phase 1 Implementation

This document outlines the **Phase 1 Foundation** work and the complete 6-phase roadmap for clean code refactoring in the Warp codebase.

---

## Phase 1: Foundation (Weeks 1-4)

### Goals
- Establish code quality standards across all 60+ crates
- Document architecture and best practices
- Set validation gates and PR review criteria
- Lay groundwork for future phases

### 1.1 Documentation Deliverables

**Status**: ✅ Complete (Weeks 1-2)

1. **CODE_QUALITY.md** — Standards for module organization, error handling, logging, async patterns, comments, testing, and dependencies
2. **TESTING.md** — Comprehensive testing strategy covering unit, integration, TUI, and doc tests
3. **REFACTORING_ROADMAP.md** — This document; complete 6-phase plan with timelines

### 1.2 Dependency Audit & Consolidation

**Target**: Weeks 2-3

**Task**: Move all common dependencies to `[workspace.dependencies]`

**Process**:
1. Create spreadsheet: Crate | Dependency | Current Version | Workspace Version | Status
2. Identify discrepancies (e.g., multiple versions of same dep)
3. Consolidate versions where safe, document rationale for differences
4. Update each `Cargo.toml` to use `.workspace = true`

**Expected Impact**:
- Reduce duplication by ~40-60%
- Single point of dependency updates
- Faster build times (fewer variant compilations)

**Validation**:
```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
```

### 1.3 Module Structure Audit

**Status**: In progress — baseline tooling added in `script/module-structure-lint` and findings documented in `MODULE_STRUCTURE_AUDIT.md`.

**Target**: Weeks 2-4

**Task**: Map existing crate hierarchy and identify improvement candidates

**Deliverable**: Spreadsheet with columns:
- Crate Name
- Module Depth (current vs. target)
- Public Modules (count)
- Entry Points (public API surface)
- Responsibility (brief)
- Improvement Candidate? (Y/N)
- Priority (1=High, 2=Medium, 3=Low)

**Focus Areas** (high improvement potential):
1. **crates/ai/** — 60+ modules, potential consolidation
2. **app/src/terminal/** — 150+ files, mixed concerns
3. **crates/warp_tui/** — 70+ files, UI-focused
4. **crates/warpui_core/** — 80+ files, shared UI core

**Output**: Audit report with recommendations

### 1.4 Validation Gates

**End of Phase 1 Criteria**:
- [ ] CODE_QUALITY.md approved by tech leads
- [ ] TESTING.md approved by QA lead
- [ ] All 60+ crates checked against module structure guidelines
- [ ] Dependency audit complete, rationale documented
- [ ] Zero Clippy violations workspace-wide
- [ ] All existing tests pass
- [ ] Documentation reviewed for accuracy

### 1.5 Team Communication

**Kickoff Meeting** (Week 1):
- Present new CODE_QUALITY.md and TESTING.md
- Explain phase timeline and expectations
- Answer questions on refactoring impact

**Checkpoint** (Week 3):
- Review audit findings
- Discuss prioritization with team
- Adjust Phase 2 timeline if needed

---

## Phase 2: Module Restructuring (Weeks 5-12)

### Goals
- Refactor high-priority crates to 2-3 level hierarchy
- Establish private/public boundaries
- Reduce coupling between modules
- Maintain 100% backward compatibility through deprecation

### 2.1 Priority 1: crates/ai/

**Target**: Weeks 5-8

**Current State**:
- ~60 modules (hierarchical but inconsistent)
- Mixed concerns: agent logic, orchestration, blocklist, execution
- Some overly nested branches

**Proposed Structure**:
```rust
crates/ai/src/
├── lib.rs          (re-exports only)
├── agent/
│   ├── mod.rs      (Agent type)
│   ├── agent.rs    (implementation)
│   └── agent_tests.rs
├── orchestration/
│   ├── mod.rs
│   ├── orchestrator.rs
│   └── orchestration_tests.rs
├── execution/
│   ├── mod.rs
│   ├── executor.rs
│   └── execution_tests.rs
└── types.rs        (shared types)
```

**Actions**:
1. Map existing modules to target structure
2. Create new `mod.rs` files with selective re-exports
3. Move private modules into private scope
4. Add deprecation warnings for old public paths
5. Test all consumers
6. Update internal docs

**Success Criteria**:
- Hierarchy depth: max 3 levels
- Public modules: <10
- All tests pass
- Zero new Clippy violations

### 2.2 Priority 2: app/src/terminal/

**Target**: Weeks 9-10

**Current State**:
- 150+ files
- Mix of model, view, utility, and tests
- Unclear responsibility boundaries

**Proposed Structure**:
```rust
app/src/terminal/
├── mod.rs
├── model/
│   ├── mod.rs      (TerminalModel)
│   ├── state.rs
│   ├── session.rs
│   └── model_tests.rs
├── input/
│   ├── mod.rs
│   ├── handler.rs
│   └── input_tests.rs
├── grid/
│   ├── mod.rs
│   ├── buffer.rs
│   ├── rendering.rs
│   └── grid_tests.rs
└── util/
    ├── mod.rs
    └── helpers.rs
```

**Actions**:
1. Audit current files; categorize by concern
2. Migrate to structure above
3. Update imports throughout app crate
4. Ensure all tests remain colocated
5. Document model/input/grid boundaries

**Success Criteria**:
- Hierarchy depth: max 3 levels
- Clear separation: model vs. input vs. rendering
- All imports use public API only
- Tests pass

### 2.3 Priority 3: crates/warp_tui/

**Target**: Weeks 11-12

**Current State**:
- 70+ files
- TUI-focused; mostly UI elements
- Some mixed concerns

**Proposed Structure**:
```rust
crates/warp_tui/src/
├── lib.rs
├── ui/
│   ├── mod.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── input.rs
│   │   ├── button.rs
│   │   └── components_tests.rs
│   ├── layout/
│   │   ├── mod.rs
│   │   └── layout_tests.rs
│   └── ui_tests.rs
├── input/
│   ├── mod.rs
│   ├── handler.rs
│   └── input_tests.rs
├── session/
│   ├── mod.rs
│   └── session_tests.rs
└── views/
    ├── mod.rs
    └── views_tests.rs
```

**Actions**:
1. Group files by concern (UI, input, session, views)
2. Create `mod.rs` files with clear public APIs
3. Migrate tests to `*_tests.rs` pattern
4. Remove redundant module nesting
5. Document view hierarchy

**Success Criteria**:
- Hierarchy depth: max 3 levels
- Public API minimal and clear
- All tests pass
- No regression in TUI rendering

### 2.4 Refactoring Workflow

**For each crate**:

1. **Create feature branch**:
   ```bash
   git checkout -b refactor/ai-module-restructure
   ```

2. **Document old structure** (for PR reviewers)

3. **Execute restructuring** (preserve all logic):
   - Move files
   - Update module paths
   - Create new `mod.rs` files
   - Add deprecation notices where needed

4. **Run validation**:
   ```bash
   ./script/format
   cargo clippy --workspace -- -D warnings
   cargo nextest run -p <crate>
   ```

5. **Create PR** with:
   - Before/after module tree
   - List of public API changes
   - Links to new documentation
   - Test results

6. **Post-merge**:
   - Mark deprecations for cleanup
   - Monitor for consumer breakage
   - Update internal tooling if needed

### 2.5 Validation Gates

**End of Phase 2 Criteria**:
- [ ] All three priority crates refactored
- [ ] Hierarchy depth: max 3 levels across all crates
- [ ] Public module count reduced by 30%+
- [ ] All tests pass locally and in CI
- [ ] Zero new Clippy violations
- [ ] Documentation updated
- [ ] Team review complete

---

## Phase 3: Error Handling Standardization (Weeks 13-16)

### Goals
- Consolidate error types across codebase
- Establish clear error propagation patterns
- Improve error reporting and diagnostics
- Reduce error-handling boilerplate

### 3.1 Extend warp_errors Crate

**Task**: Expand existing `crates/warp_errors/` with domain-specific errors

**Current**:
- Error registration for Sentry
- Some error type implementations

**Add**:
```rust
// crates/warp_errors/src/domain_errors.rs
pub enum DomainError {
    Terminal(TerminalError),
    AI(AIError),
    Cloud(CloudError),
    IO(std::io::Error),
    // ...
}

pub enum TerminalError {
    PTYFailed(String),
    EncodingError { input: Vec<u8> },
    CommandTimeout { duration: Duration },
}

pub enum AIError {
    ExecutionFailed(String),
    ModelUnavailable,
    InvalidConfiguration(String),
}

pub trait ErrorWithContext<T> {
    fn with_context(self, context: impl std::fmt::Display) -> Result<T>;
}
```

### 3.2 Migration Strategy

**Phase 3a** (Weeks 13-14): Terminal errors
- Convert terminal I/O errors to `TerminalError`
- Update error propagation in `crates/warp_terminal/`
- Add tests for error cases

**Phase 3b** (Weeks 15-16): AI errors
- Convert AI execution errors to `AIError`
- Update `crates/ai/` error handling
- Add tests for error cases

### 3.3 Documentation

**Add to CODE_QUALITY.md**:
- When to use domain errors vs. `anyhow`
- Error propagation decision tree
- Examples for each domain

---

## Phase 4: Testing & Coverage (Weeks 17-20)

### Goals
- Improve test organization
- Increase coverage to 80%+
- Document testing patterns

### 4.1 Test Infrastructure

**Task**: Create `crates/test_utils/` for shared test helpers

**Contents**:
```rust
pub mod fixtures;    // Test data builders
pub mod mocks;       // Common mock implementations
pub mod assertions;  // Custom assertion macros
```

### 4.2 Coverage Reporting

**Setup**:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage --workspace
```

**Target**: 80% coverage on business logic

### 4.3 High-Priority Areas

- AI orchestration (currently ~60% coverage)
- Terminal state transitions (currently ~70% coverage)
- Error recovery paths (currently ~50% coverage)

---

## Phase 5: Async & Concurrency (Weeks 21-24)

### Goals
- Audit all locking patterns
- Reduce deadlock risk
- Document async best practices

### 5.1 Lock Audit

**Task**: Map all `.lock()` calls in codebase

**Deliverable**: Spreadsheet with:
- File | Line | Lock Type | Frequency | Risk Level

**Risk Assessment**:
- High: TerminalModel locks from multiple call sites
- Medium: Long-held locks, potential contention
- Low: Short-lived, clear ordering

### 5.2 Refactoring Critical Paths

**Focus**: TerminalModel (per AGENTS.md warning)

- Document lock hierarchy
- Reduce lock scope where possible
- Use `Arc<RwLock<>>` instead of `Mutex` where appropriate
- Add lock tracing for debugging

---

## Phase 6: Documentation (Weeks 25-28)

### Goals
- Complete high-level documentation
- Create architectural decision records (ADRs)
- Write per-crate READMEs

### 6.1 Update AGENTS.md

- Add references to CODE_QUALITY.md
- Add references to TESTING.md
- Update examples with modern patterns
- Add section on refactoring history

### 6.2 Module READMEs

**Priority crates**:
- `crates/ai/README.md` — Agent architecture, usage examples
- `crates/warp_terminal/README.md` — Terminal model design, PTY handling
- `crates/warpui_core/README.md` — Shared UI core, entity-handle system
- `crates/warp_tui/README.md` — TUI rendering, element library

**Template**:
```markdown
# Crate Name

## Overview
Brief description of purpose and responsibilities.

## Architecture
High-level design and key components.

## Public API
Minimal set of exported types/functions.

## Testing
How to test this crate.

## Performance Considerations
Known bottlenecks, optimization strategies.
```

### 6.3 Architectural Decision Records (ADRs)

**Format**: `ADR-NNN-title.md`

**Examples**:
- ADR-001: Entity-Handle System over direct ownership
- ADR-002: Workspace monorepo over separate repos
- ADR-003: Feature flags for platform-specific code
- ADR-004: Async/await with tokio over threads
- ADR-005: Error types: domain-specific vs. generic

---

## Success Metrics

### Code Quality

| Metric | Baseline | Target | Timeline |
|--------|----------|--------|----------|
| Avg Module Depth | 4-5 | 2-3 | Week 12 |
| Public Modules | 100+ | 40-50 | Week 12 |
| Test Coverage | ~70% | 80%+ | Week 20 |
| Clippy Violations | 0 | 0 | Ongoing |
| Documentation Coverage | ~60% | 100% | Week 28 |

### Developer Experience

| Metric | Current | Target |
|--------|---------|--------|
| Time to onboard new dev | 2 weeks | 1 week |
| Time to add new feature | Variable | Consistent |
| PR review time | 2-3 days | 1 day |
| Build time (incremental) | ~30s | <20s |

---

## Risk Mitigation

### Large Refactoring Risks

| Risk | Mitigation |
|------|-----------|
| Breaking changes in public APIs | Deprecation warnings; staged rollout; feature branches |
| Merge conflicts | Small, frequent PRs; coordinate with team |
| Regression in performance | Benchmarking before/after; focus on module boundaries |
| Developer disruption | Clear communication; pair programming; staged rollout |

### Validation Strategy

**Per-phase validation**:
1. All tests pass locally
2. All tests pass in CI
3. No new Clippy violations
4. Code review by 2+ team members
5. Performance benchmarks stable
6. Documentation complete

---

## CI/CD Integration

### Expanded Presubmit Checks

**Add to `script/presubmit`**:
```bash
# Coverage check (new)
cargo tarpaulin --out Json --output-dir coverage

# Module structure lint (new)
./script/module-structure-lint

# Deadlock detection (new)
./script/detect-suspicious-locks
```

### PR Checklist Bot

**Automated checks**:
- ✓ Clippy passes
- ✓ Tests pass
- ✓ Coverage >80% (business logic)
- ✓ No new public modules (unless justified)
- ✓ Comments explain "why" not "what"
- ✓ CODE_QUALITY.md compliance

---

## Timeline Overview

```
Phase 1: Foundation
  ├─ Week 1-2: Documentation (CODE_QUALITY.md, TESTING.md)
  ├─ Week 2-3: Dependency consolidation
  ├─ Week 3-4: Module audit
  └─ Gate: Team approval of standards

Phase 2: Module Restructuring
  ├─ Week 5-8:   Priority 1 (crates/ai/)
  ├─ Week 9-10:  Priority 2 (app/src/terminal/)
  ├─ Week 11-12: Priority 3 (crates/warp_tui/)
  └─ Gate: All tests pass, hierarchy targets met

Phase 3: Error Handling
  ├─ Week 13-14: Terminal errors
  ├─ Week 15-16: AI errors
  └─ Gate: 80%+ error path coverage

Phase 4: Testing
  ├─ Week 17-18: Test infrastructure
  ├─ Week 19-20: Coverage improvement
  └─ Gate: 80%+ overall coverage

Phase 5: Async & Concurrency
  ├─ Week 21-22: Lock audit
  ├─ Week 23-24: Critical path refactoring
  └─ Gate: Lock hierarchy documented, no new deadlock risks

Phase 6: Documentation
  ├─ Week 25-26: ADRs and AGENTS.md updates
  ├─ Week 27-28: Module READMEs
  └─ Gate: All documentation complete

Buffer: Weeks 28-32
  └─ Contingency, reviews, final polish
```

---

## Next Steps

1. **Immediate (This Week)**:
   - Review CODE_QUALITY.md and TESTING.md
   - Schedule team kickoff meeting
   - Begin dependency audit

2. **Week 2**:
   - Finalize documentation with team feedback
   - Complete dependency consolidation
   - Begin module structure audit

3. **Week 3**:
   - Present audit findings
   - Prioritize Phase 2 crates with team
   - Plan first refactoring PR

4. **Week 5**:
   - Begin Phase 2 refactoring
   - Maintain velocity on bug fixes
   - Monitor build/test performance

---

## References

- **CODE_QUALITY.md**: Module organization, error handling, logging, async, comments, testing, dependencies
- **TESTING.md**: Unit, integration, TUI, and doc testing patterns
- **AGENTS.md**: Existing development guidelines (comprehensive reference)
- **in-repo-rust-unit-tests skill**: Unit testing patterns specific to Warp
- **gui-integration-test skill**: GUI integration test framework
- **tui-testing skill**: TUI render-to-lines patterns

---

**Version**: 1.0
**Last Updated**: 2025
**Owner**: Warp Development Team
**Status**: Ready for Phase 1 Implementation
