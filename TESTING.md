# Warp Testing Strategy & Patterns

This document provides comprehensive guidance on testing across the Warp codebase, covering unit tests, integration tests, TUI render tests, and common testing patterns.

---

## 1. Testing Levels & When to Use Each

### 1.1 Unit Tests (Fast, Deterministic, Isolated)

**Best for**:
- Pure functions and data transformations
- Domain logic and state machines
- Parsing and validation
- Error handling and edge cases
- Bug fixes at the logic level

**Location**: Colocated with source in `*_tests.rs` files

**Example**:
```rust
#[test]
fn parses_command_with_pipes_correctly() {
    let input = "cat file.txt | grep pattern";
    let ast = parse(input).unwrap();
    assert_eq!(ast.len(), 2);
    assert_eq!(ast[0].command, "cat");
    assert_eq!(ast[1].command, "grep");
}
```

**Run**:
```bash
cargo nextest run -p example_crate
cargo nextest run -E 'test(parse)'  # Filter by name
```

### 1.2 Integration Tests (GUI)

**Best for**:
- End-to-end behavior (terminal + shell + UI wiring)
- Settings and keybinding effects
- Focus movement and selection
- Real PTY interaction

**Location**: `crates/integration/` (GUI-specific framework)

**Example** (from `gui-integration-test` skill):
```rust
#[test]
fn typing_text_appears_in_terminal() {
    test_app(|mut app| async move {
        initialize_app_for_terminal_view(&mut app);
        let term = add_window_with_terminal(&mut app, None);

        // Type into terminal
        term.update(&mut app, |view, ctx| {
            view.handle_key_down(Key::Char('h'), ctx);
            view.handle_key_down(Key::Char('i'), ctx);
        });

        // Verify text appears
        term.read(&app, |view, _ctx| {
            assert!(view.output_contains("hi"));
        });
    })
}
```

**Run**:
```bash
cargo nextest run -p integration
```

### 1.3 TUI Render Tests

**Best for**:
- Cell-grid element rendering
- Layout calculation
- Text wrapping and alignment
- TUI input handling

**Location**: `crates/warp_tui/` and `crates/warpui_core/src/elements/tui/`

**Example** (render-to-lines pattern):
```rust
use warpui_core::elements::tui::test_support::render_to_lines;

#[test]
fn renders_input_field_with_placeholder() {
    let mut buffer = TuiBuffer::new(10, 5);
    let input = InputField::new("Name: ");

    render_to_lines(&input, &mut buffer);

    let lines = buffer.to_lines();
    assert_eq!(lines[0], "Name: ");
}
```

**Run**:
```bash
cargo nextest run -p warp_tui
```

### 1.4 Doc Tests

**Best for**:
- Documenting public API behavior
- Keeping examples up-to-date
- Simple usage patterns

**Example**:
```rust
/// Executes a shell command and returns the output.
///
/// # Examples
/// ```
/// # use warp_terminal::execute;
/// let output = execute("echo hello").unwrap();
/// assert_eq!(output.trim(), "hello");
/// ```
pub async fn execute(cmd: &str) -> Result<String> {
    // ...
}
```

**Run**:
```bash
cargo test --doc
```

---

## 2. Unit Test Structure & Best Practices

### 2.1 Test File Organization

**Small modules** (≤ 500 lines):
```rust
// src/parser.rs
pub fn parse(input: &str) -> Result<Ast> { /* ... */ }

#[cfg(test)]
#[path = "parser_tests.rs"]
mod tests;
```

**Larger modules** (split concerns):
```
src/
├── terminal/
│   ├── mod.rs
│   ├── model.rs
│   ├── input.rs
│   ├── model_tests.rs
│   └── input_tests.rs
```

### 2.2 Test Naming

**Pattern**: `test_{behavior}` or just the behavior name

```rust
#[test]
fn parses_utf8_correctly() {
    // Good: describes expected behavior
}

#[test]
fn returns_error_for_invalid_utf8() {
    // Good: describes behavior and condition
}

#[test]
fn empty_input_returns_empty_result() {
    // Good: edge case behavior
}

// Avoid
#[test]
fn test_parse() {
    // Too vague
}

#[test]
fn parse_works() {
    // Unclear what's being tested
}
```

### 2.3 Test Structure (Arrange-Act-Assert)

```rust
#[test]
fn terminal_executes_command_and_captures_output() {
    // === ARRANGE ===
    let cmd = "echo 'hello world'";
    let terminal = TerminalModel::new();

    // === ACT ===
    terminal.execute(cmd).unwrap();
    terminal.wait_for_completion(Duration::from_secs(1)).unwrap();

    // === ASSERT ===
    assert_eq!(terminal.last_output(), "hello world\n");
}
```

### 2.4 Assertion Best Practices

**Prefer `assert_eq!` / `assert_ne!` for readable diffs**:
```rust
// Good: shows diff on failure
assert_eq!(actual, expected);

// Less readable
assert!(actual == expected);

// With message for context
assert_eq!(
    actual, expected,
    "Command output should match expected for input {input:?}"
);
```

**No logic in assertions**:
```rust
// Good: expected value literal
assert_eq!(result.count, 3);

// Avoid: computed expectation (can hide bugs)
assert_eq!(result.count, items.len() / 2 + 1);
```

### 2.5 Test Doubles: Preference Hierarchy

1. **Real implementation** (default)
   ```rust
   let parser = CommandParser::new();
   let result = parser.parse("echo hello");
   assert!(result.is_ok());
   ```

2. **Repo-provided fakes** (maintained alongside real code)
   ```rust
   let terminal = TerminalModel::mock();
   terminal.simulate_block("ls", "file.txt\ndir/");
   assert!(terminal.output_contains("file.txt"));
   ```

3. **Stubbed return values** (for rare error paths only)
   ```rust
   let mut handler = Handler::new();
   handler.stub_error(IoError::PermissionDenied);
   let result = handler.process();
   assert!(result.is_err());
   ```

4. **Mocks/assertions** (last resort; only for unobservable side effects)
   ```rust
   let mut mock = MockLogger::new();
   mock.expect_log("error").once();

   function_under_test(&mock);

   mock.verify();  // Must use all expected calls
   ```

### 2.6 Testing Async Code

**Use `#[tokio::test]` for async logic**:
```rust
#[tokio::test]
async fn fetch_data_with_timeout() {
    let result = fetch_with_timeout(Duration::from_secs(5)).await;
    assert!(result.is_ok());
}
```

**Use `App::test` for terminal/UI testing**:
```rust
#[test]
fn terminal_model_updates_state() {
    use warpui::App;

    App::test((), |mut app| async move {
        initialize_app_for_terminal_view(&mut app);
        let term = add_window_with_terminal(&mut app, None);

        term.update(&mut app, |view, _ctx| {
            view.model.lock().simulate_block("cmd", "output");
        });

        term.read(&app, |view, _ctx| {
            assert_eq!(view.model.lock().output(), "output");
        });
    })
}
```

### 2.7 Determinism & Avoiding Flakiness

**Never read system clock**:
```rust
// Good: inject time
#[test]
fn timeout_expires_after_duration() {
    let clock = MockClock::new();
    clock.set_now(Instant::now());

    let result = operation_with_timeout(&clock, Duration::from_secs(1));
    clock.advance(Duration::from_secs(2));

    assert!(result.is_err());  // Timeout reached
}

// Avoid
#[test]
fn timeout_expires_after_duration() {
    let start = Instant::now();
    std::thread::sleep(Duration::from_secs(2));
    // Flaky: depends on system speed
}
```

**Never sleep to wait for async**:
```rust
// Good: await the future
#[tokio::test]
async fn waits_for_operation() {
    let result = async_operation().await;
    assert!(result.is_ok());
}

// Avoid
#[test]
fn waits_for_operation() {
    spawn_async_operation();
    std::thread::sleep(Duration::from_millis(100));
    // Flaky: timing-dependent
}
```

**Tests must pass in any order**:
```rust
// Good: isolated state
#[test]
fn can_parse_config() {
    let result = parse_config("test_config.toml");
    assert!(result.is_ok());
}

// Avoid: depends on test execution order
static mut GLOBAL_STATE: Option<Config> = None;

#[test]
fn initializes_global() {
    unsafe { GLOBAL_STATE = Some(Config::new()); }
}

#[test]
fn uses_global() {
    let cfg = unsafe { GLOBAL_STATE.as_ref().unwrap() };
    // Fails if this test runs before initialize_global
}
```

---

## 3. Common Test Helpers & Utilities

### 3.1 Terminal Model Testing

```rust
use warp_terminal::TerminalModel;

// Create a test terminal
let terminal = TerminalModel::mock();

// Simulate command execution
terminal.simulate_block("echo hello", "hello");

// Simulate command with error
terminal.simulate_cmd_with_error("false", "error message");

// Get state
let blocks = terminal.block_list();
let output = terminal.output();

// Finish block (mark as done)
terminal.finish_block();
```

### 3.2 Data Builders

**Use builders for focused test setup**:
```rust
use warp_terminal::model::test_utils::{TestBlockBuilder, TestBlockListBuilder};

let block = TestBlockBuilder::default()
    .command("echo hello")
    .output("hello")
    .duration_secs(0.1)
    .build();

let block_list = TestBlockListBuilder::default()
    .with_block(block)
    .with_block(TestBlockBuilder::default().command("pwd").build())
    .build();
```

### 3.3 Virtual Filesystem

**For I/O-heavy logic**:
```rust
use virtual_fs::{VirtualFS, Stub};

#[test]
fn reads_config_from_file() {
    VirtualFS::test("config_test", |_dirs, mut fs| {
        fs.with_files(vec![
            Stub::FileWithContent("config.toml", "[settings]\nkey=value"),
        ]);

        let config = read_config("config.toml");
        assert_eq!(config.get("key"), Some("value"));
    })
}
```

### 3.4 Feature Flags

**Override at test time**:
```rust
use warp_features::FeatureFlag;

#[test]
fn feature_behavior_when_enabled() {
    let _flag = FeatureFlag::CreatingSharedSessions.override_enabled(true);

    let result = operation();
    assert!(result.includes_shared_session_feature);
}

#[test]
fn feature_behavior_when_disabled() {
    let _flag = FeatureFlag::CreatingSharedSessions.override_enabled(false);

    let result = operation();
    assert!(!result.includes_shared_session_feature);
}
```

### 3.5 Custom Test Fixtures

**For reusable test data**:
```rust
mod fixtures {
    use crate::*;

    pub fn sample_command() -> Command {
        Command::new("echo", vec!["hello"])
    }

    pub fn sample_block() -> Block {
        Block::new(sample_command(), "hello\n", Duration::from_millis(100))
    }
}

#[test]
fn executes_sample_block() {
    let block = fixtures::sample_block();
    assert_eq!(block.output, "hello\n");
}
```

---

## 4. TUI Render Testing

**Reference**: `tui-testing` skill for detailed patterns.

### 4.1 Basic Render Test

```rust
use warpui_core::elements::tui::{TuiElement, TuiBuffer, test_support::render_to_lines};

#[test]
fn renders_text_element() {
    let text = Text::new("Hello, Warp!");
    let mut buffer = TuiBuffer::new(20, 1);

    text.render(&mut buffer, Rect::new(0, 0, 20, 1));

    let lines = buffer.to_lines();
    assert_eq!(lines[0], "Hello, Warp!");
}
```

### 4.2 Layout Tests

```rust
#[test]
fn layouts_children_in_row() {
    let row = Row::new(vec![
        Box::new(Text::new("Left")),
        Box::new(Text::new("Right")),
    ]);

    let mut buffer = TuiBuffer::new(20, 1);
    row.render(&mut buffer, Rect::new(0, 0, 20, 1));

    let lines = buffer.to_lines();
    assert!(lines[0].contains("Left"));
    assert!(lines[0].contains("Right"));
}
```

### 4.3 Input Handling

```rust
#[test]
fn input_field_captures_typed_text() {
    let mut input = InputField::new();

    input.handle_input(TuiEvent::Char('a'));
    input.handle_input(TuiEvent::Char('b'));

    assert_eq!(input.value(), "ab");
}
```

---

## 5. Integration Test Framework (GUI)

**Reference**: `gui-integration-test` skill for full details.

### 5.1 Basic Integration Test

```rust
#[test]
fn terminal_view_responds_to_user_input() {
    test_app(|mut app| async move {
        // Setup
        initialize_app_for_terminal_view(&mut app);
        let term = add_window_with_terminal(&mut app, None);

        // Interact
        term.update(&mut app, |view, ctx| {
            view.input_handler.key_pressed(Key::Char('l'), ctx);
            view.input_handler.key_pressed(Key::Char('s'), ctx);
        });

        // Verify
        term.read(&app, |view, _ctx| {
            let output = view.model.lock().output();
            assert!(output.contains("ls"));
        });
    })
}
```

### 5.2 Async Integration Test

```rust
#[test]
fn command_execution_completes_successfully() {
    test_app(|mut app| async move {
        initialize_app_for_terminal_view(&mut app);
        let term = add_window_with_terminal(&mut app, None);

        term.update(&mut app, |view, _ctx| {
            view.model.lock().execute("sleep 0.1");
        });

        // Wait for completion
        tokio::time::sleep(Duration::from_millis(200)).await;

        term.read(&app, |view, _ctx| {
            let model = view.model.lock();
            assert!(model.last_block().is_finished());
        });
    })
}
```

---

## 6. Writing New Tests: Quick Checklist

- [ ] **Is the behavior reachable without booting the app?** → Unit test
- [ ] **Does it require PTY/shell/real window/display?** → Integration test
- [ ] **Is it pure logic?** → Unit test; use real implementations
- [ ] **Rare error path?** → Unit test; stub return value
- [ ] **End-to-end UI behavior?** → Integration test
- [ ] **TUI rendering/layout?** → TUI render test
- [ ] **Test name describes *behavior*?** → ✓
- [ ] **Arrange-Act-Assert structure?** → ✓
- [ ] **Deterministic?** → ✓ (no sleeps, clocks, or test order deps)
- [ ] **Colocated in `*_tests.rs`?** → ✓
- [ ] **Passes locally?** → ✓ (run `cargo nextest run`)

---

## 7. Running Tests Locally

### 7.1 Full Presubmit

```bash
./script/presubmit
```

Runs formatting, clippy, and all tests.

### 7.2 Workspace Tests

```bash
# All tests in parallel
cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2

# Single crate
cargo nextest run -p warp_terminal

# Filter by name
cargo nextest run -E 'test(parse)'

# With logging (debug level)
RUST_LOG=debug cargo nextest run -p warp_terminal
```

### 7.3 Doc Tests

```bash
cargo test --doc
```

### 7.4 Watch Mode (development)

```bash
# Install cargo-watch if needed
cargo install cargo-watch

# Re-run tests on file changes
cargo watch -x "nextest run -p warp_terminal"
```

---

## 8. Troubleshooting Tests

### Test hangs or deadlocks

**Likely cause**: Multiple `.lock()` calls on same model in call stack.

**Fix**: Pass already-locked reference instead of re-locking.

```rust
// Bad
fn process(terminal: &TerminalModel) {
    terminal.lock().step1();  // Lock 1
    helper(terminal);         // Lock 2 — deadlock
}

fn helper(terminal: &TerminalModel) {
    terminal.lock().step2();  // Waits forever
}

// Good
fn process(terminal: &TerminalModel) {
    let mut model = terminal.lock();
    model.step1();
    helper(&mut model);  // Pass reference, no new lock
}

fn helper(model: &mut TerminalModel) {
    model.step2();
}
```

### Test fails intermittently

**Likely cause**: Time dependency, async race, or test order.

**Fix**: Determinism checklist:
- No `Instant::now()` or `SystemTime::now()` — inject time
- No `std::thread::sleep()` — use `.await` or channels
- No global state — use `#[serial]` if unavoidable
- No test-order dependencies — each test standalone

### Assertion produces confusing diff

**Fix**: Use `assert_eq!` instead of `assert!`, add message:

```rust
assert_eq!(
    actual, expected,
    "Terminal output should contain command:\ninput={input:?}"
);
```

---

## 9. Test Coverage Targets

**Aim for**:
- **80%+ coverage** on business logic
- **60%+ coverage** on UI/rendering (hard to test exhaustively)
- **100% coverage** on error paths (especially error types)

**Run coverage locally** (if `tarpaulin` installed):
```bash
cargo tarpaulin --out Html --output-dir coverage --workspace
```

---

## References

- **CODE_QUALITY.md**: Code style and organization
- **in-repo-rust-unit-tests skill**: Unit testing patterns
- **gui-integration-test skill**: GUI integration framework
- **tui-testing skill**: TUI render-to-lines patterns
- **AGENTS.md**: General development guidelines

---

**Version**: 1.0
**Last Updated**: 2025
**Owner**: Warp Development Team
