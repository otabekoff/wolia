# GitHub Copilot Instructions for Wolia

This file provides instructions for GitHub Copilot when working on the Wolia codebase.

## Project Overview

Wolia is a modern, GPU-accelerated office suite built in Rust. It's a **Cargo workspace** with multiple crates organized into:

- **Apps**: End-user applications (wolia-write, wolia-grid, wolia-deck)
- **Engine**: Shared core functionality (core, layout, render, edit, format, platform, math, assets, plugin)
- **Engine Modules**: App-specific engines (grid-engine, deck-engine)
- **Formats**: File format implementations (wolia-format, docx, xlsx, pptx, pdf, markdown)
- **Plugins**: Official plugins (latex, diagrams, code-blocks)
- **Tooling**: Development tools (font-processor, asset-pipeline, test-generator, fuzzers)

## Technology Stack

- **Language**: Rust 2024 edition (requires Rust 1.85+)
- **Build System**: Cargo
- **Graphics**: wgpu for GPU rendering
- **Windowing**: winit
- **Text Rendering**: cosmic-text, fontdb, rustybuzz
- **Async Runtime**: tokio
- **Testing**: standard Rust tests, criterion for benchmarks, proptest for property testing, insta for snapshot testing

## Development Workflow

### Must haves

- Don't just finish tasks, don't just finish tasks to makr them as done, finish them to make really work.
- Never be lazy and don't avoid doing hard things and don't do easy ones instead. Eyes are coward, hads are brave.
- Don't simplify to satisfy. Don't simlify tests if they complain or give error, find exact cause and fix them.
- Don't add `2>&1`, `tail` and `head` to commands.
- Make sure there is no problems left in problems tab at the end of task.
- At the end of the task do use clippy, deny, test, check commands and if problems, fix them.
- At the very end of task, if successful, do commit conventionally according to Google's conventional commit.
- Don't forget to update the tracking file like ROADMAP.md as you make progress.
- Don't create summary documents or no need to tell them at the end of the work.

### Before Making Changes

1. **Understand the workspace structure**: This is a Cargo workspace with many interconnected crates
2. **Check existing issues and PRs**: Avoid duplicate work
3. **Run initial checks**:
   ```bash
   cargo check       # Quick type check
   cargo clippy      # Lint
   cargo test        # Run tests
   ```

### Making Changes

1. **Make minimal, focused changes**: Keep changes surgical and precise
2. **Follow Rust conventions**: See AGENTS.md for detailed style guidelines
3. **Update tests**: Add or modify tests for your changes
4. **Check documentation**: Update docs if changing public APIs

### Before Committing

1. **Format code**: `cargo fmt`
2. **Fix lints**: `cargo clippy` (should pass without warnings)
3. **Run tests**: `cargo test` (all tests should pass)
4. **Check for problems**: Review any IDE warnings or errors
5. **Use conventional commits**: Follow conventional commit format

### Build and Test Commands

```bash
# Format and lint
cargo fmt                    # Format all code
cargo clippy                 # Lint all crates

# Build
cargo build                  # Debug build
cargo build --release        # Release build
cargo check                  # Quick type check

# Test
cargo test                   # Run all tests
cargo test -p crate-name     # Test specific crate
cargo test test_name         # Run specific test

# Run applications
cargo run -p wolia-write     # Run word processor
cargo run -p wolia-grid      # Run spreadsheet
cargo run -p wolia-deck      # Run presentations
```

## Code Style Guidelines

### General Principles

- **Minimal changes**: Make the smallest changes necessary
- **Type safety**: Leverage Rust's type system
- **Error handling**: Use `Result<T, E>` and `Option<T>`, avoid `unwrap()` in production code
- **Documentation**: Document public APIs with `///` doc comments
- **Testing**: Write tests for new functionality

### Naming Conventions

- **Variables/functions**: `snake_case`
- **Types/structs/enums/traits**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

### Import Organization

Group imports in this order:

1. `std` imports
2. External crate imports
3. Internal crate imports (`crate::`)

```rust
use std::collections::HashMap;

use serde::Deserialize;
use tokio::runtime::Runtime;

use crate::core::Document;
use crate::layout::Layout;
```

### Error Handling

- Use `Result<T, E>` for recoverable errors
- Use `Option<T>` for optional values
- Propagate errors with `?` operator
- Use `thiserror` for custom error types
- Avoid `unwrap()` except in tests

### Documentation

- Document all public APIs
- Include examples in doc comments
- Keep documentation up to date with code changes

````rust
/// Renders a document to the screen.
///
/// # Arguments
///
/// * `document` - The document to render
/// * `viewport` - The viewport dimensions
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if rendering fails.
///
/// # Examples
///
/// ```
/// let doc = Document::new();
/// let viewport = Viewport::new(800, 600);
/// render(&doc, &viewport)?;
/// ```
pub fn render(document: &Document, viewport: &Viewport) -> Result<(), RenderError> {
    // ...
}
````

## Architecture Considerations

### Workspace Dependencies

- Internal dependencies are defined in `Cargo.toml` workspace dependencies
- Use workspace-level version management for shared dependencies
- Prefer workspace dependencies over duplicating versions

### Module Organization

- Engine crates provide core functionality
- Apps depend on engine crates
- Plugins extend functionality without modifying core
- Formats handle file I/O

### Performance

- This is a GPU-accelerated application
- Consider performance implications of changes
- Use benchmarks (`cargo bench`) for performance-critical code
- Profile before optimizing

## Testing Strategy

### Unit Tests

- Place unit tests in the same file as the code
- Use `#[cfg(test)]` module
- Test both success and error cases

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new();
        assert!(doc.is_empty());
    }
}
```

### Integration Tests

- Place integration tests in `tests/` directory
- Test interactions between crates
- Test user-facing functionality

### Running Tests

```bash
cargo test                           # All tests
cargo test -p wolia-core            # Tests for specific crate
cargo test -- --nocapture           # Show stdout
cargo test test_name -- --exact     # Exact test match
```

## Common Tasks

### Adding a New Feature

1. Identify the appropriate crate
2. Write tests first (TDD approach)
3. Implement the feature
4. Update documentation
5. Run `cargo fmt`, `cargo clippy`, `cargo test`

### Fixing a Bug

1. Write a failing test that reproduces the bug
2. Fix the bug
3. Verify the test passes
4. Run full test suite
5. Update relevant documentation

### Refactoring

1. Ensure tests exist for the code being refactored
2. Make changes incrementally
3. Run tests after each change
4. Keep commits small and focused

## Important Notes

- **Don't add shell redirections**: Avoid `2>&1`, `tail`, `head` in commands
- **Check problems tab**: Always review IDE warnings and errors
- **Conventional commits**: Use conventional commit format
- **Final checks**: Always run `cargo fmt`, `cargo clippy`, `cargo test` before committing
- **GPU considerations**: Remember this is GPU-accelerated, test rendering changes on real hardware when possible
- **Cross-platform**: Test changes work on Linux, macOS, and Windows
- **No unsafe unless necessary**: Avoid `unsafe` code unless absolutely required

## Resources

- **AGENTS.md**: Detailed code style guidelines
- **README.md**: Project overview and quick start
- **CONTRIBUTING.md**: Contribution guidelines
- **ROADMAP.md**: Future plans and features
- **SECURITY.md**: Security policy and reporting

## Workspace Structure Reference

```
wolia/
├── apps/              # Applications
├── engine/            # Core engine
├── engine-modules/    # App-specific engines
├── formats/           # File formats
├── plugins/           # Plugins
├── tooling/           # Dev tools
├── benchmarks/        # Performance benchmarks
├── docs/              # Documentation
└── test-suite/        # Test utilities
```
