# Warp Code Quality & Clean Code Guidelines

This document establishes standards for code organization, error handling, logging, async patterns, testing, and comments across the Warp codebase. All developers should reference this guide when writing or reviewing code.

---

## 1. Module Organization & Public APIs

### 1.1 Module Hierarchy Depth

**Rule**: Limit module nesting to **2-3 levels maximum**.

**Example (Good)**:
```rust
// crates/ai/src/lib.rs
pub mod agent;           // Level 1: Top-level public module
pub mod orchestration;
pub mod blocklist;

// crates/ai/src/agent/mod.rs
pub use self::conversation::Conversation;  // Level 2: Selective re-exports
pub use self::task::Task;

mod conversation;        // Level 2: Private submodules
mod task;
mod execution;
```

**Example (Avoid)**:
```rust
// Over-nested: too many levels
crates/ai/src/
  agents/
    core/
      models/
        execution/
          models/
            state/
              internal/
```

**Why**: Deep nesting obscures structure, slows navigation, and indicates mixed concerns. Each level should represent a conceptual boundary.

### 1.2 Minimize Public Module Surface

**Rule**: Use `pub use` for selective re-exports. Hide implementation modules.

**Pattern**:
```rust
// crates/ai/src/lib.rs - Minimal public API
pub use self::agent::{Agent, AgentConfig};
pub use self::orchestration::{Orchestrator, OrchestrationResult};

mod agent;
mod orchestration;
mod internal_helpers;  // Private
```

**Why**: Reduces coupling, enables internal refactoring without breaking consumers.

**Existing Guidance** (from `app/src/lib.rs`):
> "PLEASE DO NOT ADD MORE PUBLIC MODULES!"

We reinforce this principle for all crates.

### 1.3 Module File Organization

**For small modules** (≤ 500 lines):
```rust
// mod.rs
pub struct MyType { /* ... */ }
pub fn my_func() { /* ... */ }
fn internal_helper() { /* ... */ }

#[cfg(test)]
#[path = "my_type_tests.rs"]
mod tests;
```

**For larger modules** (> 500 lines, multiple responsibilities):
```
crates/ai/src/agent/
├── mod.rs              (re-exports only)
├── agent.rs            (Agent type & methods)
├── conversation.rs     (Conversation type & state)
├── execution.rs        (Execution logic)
├── agent_tests.rs      (Agent tests)
├── conversation_tests.rs
└── execution_tests.rs
```

**Why**: Explicit organization clarifies responsibility boundaries.

---

## 2. Error Handling

### 2.1 Standard Result Type

**Rule**: Use `anyhow::Result<T>` for most error handling. Use `warp_errors::WarpError` for domain-specific cases.

**Current State**:
- `warp_errors` crate exists with error registration patterns
- `anyhow` used workspace-wide for general errors

**When to use `WarpError`** (domain-specific):
- Terminal emulation errors (PTY failures, encoding issues)
- AI execution errors (agent failures, model errors)
- Cloud synchronization errors (conflict resolution, networking)
- Any error that must be reported to Sentry with context

**When to use `anyhow::Result<T>`** (general):
- I/O and parsing errors
- Transient network failures
- Configuration loading
- Any error not in a specialized domain

### 2.2 Error Context & Messages

**Rule**: Always provide context using `.context()` or `.with_context()`.

```rust
// Good: clear context
std::fs::read_to_string(&config_path)
    .context("Failed to read config file")?;

// Better: include relevant data
std::fs::read_to_string(&config_path)
    .context(format!("Failed to read config file at {}", config_path.display()))?;

// Avoid: no context
std::fs::read_to_string(&config_path)?;

// Avoid: error swallowing
std::fs::read_to_string(&config_path).unwrap_or_default();
```

### 2.3 Sensitive Data Redaction

**Rule**: Never log passwords, tokens, API keys, or personally identifiable information.

Use `safe_*` macros (provided by `warp_errors` or `secret_redaction`):

```rust
// Good
safe_debug!("User context: {:?}", user_context);

// Good: manual redaction
let sanitized = SanitizedString::new(&auth_token);
log::info!("Auth attempt: {}", sanitized);

// Avoid
log::info!("Authorization header: {}", auth_header);
```

### 2.4 When to Use `.unwrap()` or `.expect()`

**Never use** `.unwrap()` or `.expect()` in production code except in these cases:

1. **During initialization** (invariant that must hold):
   ```rust
   let regex = regex::Regex::new(r"^\d+$").expect("Hardcoded regex is valid");
   ```

2. **In tests**:
   ```rust
   #[test]
   fn my_test() {
       let result = operation().unwrap();  // OK in test
   }
   ```

3. **After explicit validation**:
   ```rust
   assert!(value.is_some(), "precondition checked");
   let unwrapped = value.unwrap();  // Safe after assert
   ```

**Instead**, propagate errors:
```rust
// Good
fn process(input: &str) -> Result<Output> {
    let parsed = parse(input)?;
    transform(&parsed)
}

// Avoid
fn process(input: &str) -> Output {
    let parsed = parse(input).unwrap();  // Will panic
    transform(&parsed)
}
```

---

## 3. Logging & Observability

### 3.1 Log Levels

**`log::trace!`** — Extremely detailed, rarely enabled:
- Entry/exit from hot functions
- Variable state at every step
- Allocation/deallocation tracking

**`log::debug!`** — Development-focused detail:
- Function entry with arguments
- State transitions
- Intermediate results (non-sensitive)
- Disabled in release builds (typically)

**`log::info!`** — User-visible state:
- Application startup/shutdown
- Feature enablement (agent mode, feature flags)
- Major state changes (connected, authenticated, disconnected)
- Never too frequently (avoid spam)

**`log::warn!`** — Unexpected but recoverable:
- Fallback behavior activated
- Deprecated API used
- Performance degradation detected
- Missing optional resources

**`log::error!`** — Recoverable errors:
- Failed request retry → success expected
- Optional feature unavailable
- Transient network failure

**For unrecoverable errors**, use `report_error!()` (if integrated):
```rust
use warp_errors::report_error;

match critical_operation() {
    Ok(result) => result,
    Err(e) => {
        report_error!("Critical operation failed: {e:?}");
        return Err(e);
    }
}
```

### 3.2 Format & Inline Arguments

**Rule**: Use inline format arguments for clarity.

```rust
// Good (clippy recommendation)
log::info!("User {user_id} logged in from {ip}");
eprintln!("{message}");

// Avoid (flagged by clippy)
log::info!("User {} logged in from {}", user_id, ip);
eprintln!("{}", message);
```

### 3.3 Log Per-Module Verbosity

If a module becomes noisy, consider a feature flag or runtime control:

```rust
// In code
if log::log_enabled!(log::Level::Debug) {
    log::debug!("Detailed state: {state:?}");
}

// Or via feature flag
#[cfg(feature = "debug_ai")]
log::debug!("AI reasoning: {trace}");
```

### 3.4 Structured Logging for Telemetry

When logging for analytics/observability, use structured fields:

```rust
// Good: structured for analysis
log::info!(
    "command_executed";
    "user_id" => user_id,
    "command" => &command_name,
    "duration_ms" => elapsed.as_millis(),
    "success" => success,
);

// Rather than
log::info!("User {} executed {} in {} ms", user_id, command_name, elapsed.as_millis());
```

---

## 4. Async & Concurrency Patterns

### 4.1 Lock Usage & Ordering

**Critical Warning** (from AGENTS.md):
> "Acquiring multiple locks on the same model from different call sites can cause a deadlock, resulting in a UI freeze."

**TerminalModel Lock Safety**:
- **Never** call `.lock()` twice on the same model in the same call stack
- **Always** pass an already-locked reference down:

```rust
// Good: pass locked reference
fn process(terminal: &TerminalModel) -> Result<()> {
    let mut model = terminal.lock();
    helper(&mut model)?;  // Pass already-locked
    Ok(())
}

fn helper(model: &mut TerminalModel) {
    // Uses passed reference, no new lock
    model.update_state();
}

// Avoid: re-locking
fn process(terminal: &TerminalModel) -> Result<()> {
    terminal.lock().update_state();  // Lock 1
    helper(terminal)?;  // Lock 2 in helper — DEADLOCK RISK
    Ok(())
}

fn helper(terminal: &TerminalModel) {
    terminal.lock().update_state();  // Second lock on same stack
}
```

### 4.2 Blocking Operations

**Rule**: Use `blocking::unblock()` for blocking I/O in async contexts.

```rust
async fn read_config_async(path: &Path) -> Result<String> {
    let path = path.to_path_buf();
    blocking::unblock(move || {
        std::fs::read_to_string(&path)
    })
    .await?
    .context("Failed to read config")
}
```

### 4.3 Timeouts for Long-Running Async Operations

```rust
use tokio::time::{timeout, Duration};

async fn fetch_with_timeout() -> Result<Data> {
    match timeout(Duration::from_secs(30), fetch_data()).await {
        Ok(Ok(data)) => Ok(data),
        Ok(Err(e)) => Err(e).context("Fetch failed"),
        Err(_) => Err(anyhow!("Fetch timed out after 30s")),
    }
}
```

### 4.4 Async Testing

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = my_async_function().await;
    assert!(result.is_ok());
}

// For app tests, use warpui::App::test:
#[test]
fn test_with_app() {
    use warpui::App;
    App::test((), |mut app| async move {
        // app is available in async context
        // No manual runtime setup needed
    })
}
```

---

## 5. Comments & Documentation

### 5.1 Comment Philosophy

**From AGENTS.md**: "Comments have a cost. Be judicious."

**Minimize comments that explain *what* the code does** — the code should explain itself through clear names and structure.

**Reserve comments for**:
1. **Why**: Non-obvious business logic or constraints
   ```rust
   // We cache the parsed regex to avoid recompilation on every call.
   // Regex::new is O(n) in pattern length.
   let regex = REGEX_CACHE.get_or_insert();
   ```

2. **Workarounds**: Third-party bugs or unavoidable complexity
   ```rust
   // tokio::fs::read_to_string sometimes hangs on network paths.
   // Use blocking::unblock as workaround for issue #12345.
   let content = blocking::unblock(/* ... */).await?;
   ```

3. **Edge cases**: Non-obvious behavior
   ```rust
   // Empty input should return Ok(()) not error; matches shell behavior.
   if input.trim().is_empty() {
       return Ok(());
   }
   ```

4. **Unidiomatic patterns**: When best solution isn't Rust-idiomatic
   ```rust
   // Requires unsafe for FFI interop; SAFETY checked below.
   unsafe {
       // ...
   }
   ```

### 5.2 No Line-by-Line Narration

**Avoid**:
```rust
// Initialize the count
let count = 0;

// Loop over items
for item in items {
    // Increment count
    count += 1;
}
```

These comments restate what the code already says. Delete them.

### 5.3 Doc Comments on Public APIs

**Keep concise**. Focus on intent, not implementation.

```rust
/// Parses a command string into executable blocks.
///
/// # Errors
/// Returns an error if the command contains invalid syntax.
pub fn parse_command(input: &str) -> Result<Vec<Block>> {
    // ...
}

/// Executes the given block in the terminal.
///
/// Handles PTY setup, output capture, and error recovery.
pub async fn execute(block: &Block) -> Result<Output> {
    // ...
}

// Avoid: over-documented implementation details
/// This function initializes a mutable variable `output` to an empty string,
/// then enters a loop to process each character, appending to the output variable,
/// and then returns the output variable.
pub fn process_chars(input: &str) -> String {
    // ...
}
```

### 5.4 Maintain Comments on Refactors

**Do not remove existing comments** when refactoring unrelated code. Comments exist to explain current behavior.

**Only update/remove comments if**:
- The behavior they describe has changed
- They contain outdated information
- They were wrong to begin with

**Do not add comments explaining *your edit***. Explain the current state, not the change. Edit explanations belong in PR comments.

---

## 6. Testing Standards

### 6.1 Where Tests Live

**Unit tests**: Colocated with source code in separate files.

```rust
// crates/ai/src/agent.rs
pub struct Agent { /* ... */ }

impl Agent {
    pub fn new() -> Self { /* ... */ }
}

#[cfg(test)]
#[path = "agent_tests.rs"]
mod tests;

// crates/ai/src/agent_tests.rs
#[test]
fn agent_initialization_succeeds() {
    let agent = Agent::new();
    assert!(agent.is_ready());
}
```

**Integration tests**: In `crates/integration/` (GUI-specific framework).

**TUI render tests**: Use `render_to_lines` in `crates/warp_tui` (see `tui-testing` skill).

### 6.2 Test Naming

**Name tests after the *behavior* being verified**, not the method.

```rust
#[test]
fn parses_valid_utf8_sequence() {
    // Good: describes behavior
}

#[test]
fn returns_error_for_invalid_utf8() {
    // Good: describes behavior
}

// Avoid
#[test]
fn test_parse() {
    // Unclear what's being tested
}

#[test]
fn parse_works() {
    // Vague
}
```

### 6.3 Test Structure

**Arrange / Act / Assert** pattern:

```rust
#[test]
fn executes_command_and_captures_output() {
    // Arrange
    let input = "echo hello";
    let terminal = TerminalModel::new();

    // Act
    terminal.execute(input).unwrap();

    // Assert
    assert_eq!(terminal.output(), "hello\n");
}
```

### 6.4 Determinism & Flakiness

**Never**:
- Read system clock from logic under test → inject time
- Sleep to wait for async → await the future or use callbacks
- Rely on test execution order → tests must pass in any order
- Leave global state between tests → use `serial_test` if unavoidable

**If you can't make a test deterministic quickly**, quarantine it with `#[ignore]` and link an issue.

### 6.5 Test Doubles: Prefer Real, Then Fakes

Work down this priority:

1. **Real implementation**: Use for value types and pure logic
2. **Repo-provided fakes**: `TerminalModel::mock()`, `Appearance::mock()`, `TestBlockListBuilder`
3. **Stubbed return values**: Only to reach rare error branches
4. **Mocks/assertions**: Last resort; only for unobservable side effects

```rust
// Good: use real implementation
let result = parse("input");
assert_eq!(result, expected);

// Good: use repo-provided fake
let terminal = TerminalModel::mock();
terminal.simulate_block("ls", "output");

// Avoid: heavy stubbing for tests
let mock = MockTerminal::new();
mock.expect_execute().once().returning(/* ... */);
```

---

## 7. Dependencies & Workspace Consistency

### 7.1 Workspace Dependencies

**Rule**: All common dependencies declared in `[workspace.dependencies]` in root `Cargo.toml`.

**Current Examples**:
```toml
[workspace.dependencies]
anyhow = "1.0"
log = { version = "0.4", features = ["serde", "std"] }
tokio = "1.x"
serde = { version = "1.0", features = ["derive"] }
```

**Each crate references**:
```toml
[dependencies]
anyhow.workspace = true
log.workspace = true
```

**Why**: Single source of truth, reduces version conflicts, easier updates.

### 7.2 Minimizing Public Dependencies

**Rule**: Don't expose dependencies in your public API unless necessary.

```rust
// Good: internal implementation
use itertools::Itertools;

pub fn get_items() -> Vec<Item> {
    // itertools is not in the public signature
    items.iter().unique().collect()
}

// Avoid: leaking dependency
use itertools::Itertools;

pub fn process(iter: impl Itertools) -> Vec<Item> {
    // Now itertools is part of your public API contract
    // Users must import it even if they don't use it directly
}
```

### 7.3 Feature Flags

**Use feature flags for**:
- Optional platform support (macOS, Linux, Windows, WASM)
- Optional major features (TUI vs GUI, `integration_tests`)
- Optional dependencies with significant size overhead

**Avoid**:
- Too many fine-grained flags (maintenance burden)
- Flags for every minor behavior toggle (use `FeatureFlag` runtime toggle instead)

**Document flag interdependencies**:
```toml
[features]
default = ["gui"]
gui = []
tui = []
# Both can be enabled, but at least one must be enabled
```

---

## 8. Clippy & Formatting

### 8.1 Clippy Lints

**All code must pass**:
```bash
cargo clippy --workspace --all-targets --tests -- -D warnings
```

**Common violations to fix**:
- `uninlined_format_args`: Use `"{var}"` not `"{}"` with `var`
- `unused_imports`: Remove unused imports
- `cognitive_complexity`: Break complex functions into helpers
- `too_many_arguments`: Refactor to pass a struct
- `comparison_to_empty_string`: Use `.is_empty()`

### 8.2 Formatting

**All code must pass**:
```bash
./script/format
```

**Max line width**: 100 characters (configured in `rustfmt.toml`).

**Reflow comments** to fill the full width:
```rust
// Good: uses full width
// This is a long comment that provides important context about the error
// handling strategy and should span multiple lines effectively.

// Avoid: narrow wrapping
// This is a long comment
// that wraps early and
// wastes space.
```

---

## 9. PR Review Checklist

Use this checklist when reviewing code during refactoring:

- [ ] **Module organization**: 2-3 levels deep max, public API minimal
- [ ] **Error handling**: Proper context, no `.unwrap()` in production
- [ ] **Logging**: Appropriate level, no sensitive data, inline format args
- [ ] **Async/locks**: No deadlock risk, no nested locks on same model
- [ ] **Comments**: Explain *why*, not *what*; no line-by-line narration
- [ ] **Tests**: Colocated, behavior-named, deterministic, good coverage
- [ ] **Dependencies**: Use workspace versions, minimal public exposure
- [ ] **Clippy**: `cargo clippy --workspace -- -D warnings` passes
- [ ] **Formatting**: `./script/format --check` passes
- [ ] **No unused imports**: Clean up after refactoring

---

## 10. References

- **AGENTS.md**: Comprehensive development guidance (existing)
- **in-repo-rust-unit-tests skill**: Unit testing patterns
- **TESTING.md**: Detailed testing strategies (forthcoming)
- **warp_errors**: Error handling infrastructure
- **workspace/Cargo.toml**: Dependency pinning

---

## Appendix: Example Module Structure

```rust
// crates/example/src/lib.rs
//! Example module showcasing clean code patterns.

pub use self::processor::{Processor, ProcessError};
pub use self::analyzer::Analyzer;

mod processor;
mod analyzer;
mod internal;  // Private; not re-exported

// crates/example/src/processor.rs
//! Processes input data.
//!
//! Handles parsing, validation, and transformation.

use anyhow::{anyhow, Context, Result};
use log::{debug, info};

/// Describes an error during processing.
#[derive(Debug)]
pub struct ProcessError {
    context: String,
}

/// Processes the given input.
///
/// # Errors
/// Returns an error if the input cannot be parsed or validated.
pub fn process(input: &str) -> Result<String> {
    info!("Starting process for input of length {}", input.len());

    // Validate input
    if input.is_empty() {
        return Err(anyhow!("Input cannot be empty"));
    }

    debug!("Input validated successfully");

    // Process
    let result = parse_and_transform(input)
        .context("Failed to parse and transform input")?;

    info!("Process completed successfully");
    Ok(result)
}

fn parse_and_transform(input: &str) -> Result<String> {
    // Implementation
    Ok(input.to_uppercase())
}

#[cfg(test)]
#[path = "processor_tests.rs"]
mod tests;

// crates/example/src/processor_tests.rs
#[test]
fn process_succeeds_with_valid_input() {
    let input = "hello";
    let result = super::process(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "HELLO");
}

#[test]
fn process_returns_error_for_empty_input() {
    let result = super::process("");
    assert!(result.is_err());
}
```

---

**Version**: 1.0
**Last Updated**: 2025
**Owner**: Warp Development Team
