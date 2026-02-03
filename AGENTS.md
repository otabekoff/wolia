# AGENTS.md

This file provides guidelines for AI agents working on the wolia Rust codebase.

## Project Overview
- **Language**: Rust (2024 edition)
- **Package Manager**: Cargo
- **Project Type**: Rust application/library

## Build, Lint, and Test Commands

### Build Commands
```bash
cargo build              # Debug build
cargo build --release    # Release build (optimized)
cargo check              # Quick type check without building
```

### Linting and Formatting
```bash
cargo clippy             # Lint with Clippy
cargo fmt                # Format code with rustfmt
cargo fmt --check        # Check formatting without modifying
```

### Testing Commands
```bash
cargo test                           # Run all tests
cargo test --no-run                  # Compile tests without running
cargo test --lib                     # Run library tests only
cargo test --bin wolia               # Run binary tests only
```

#### Running Single Tests
```bash
cargo test test_name                    # Run specific test by name
cargo test test_name -- --exact         # Exact match for test name
cargo test test_name -- --nocapture     # Show stdout from test
cargo test module::test_name            # Run test in specific module
cargo test -- --test-threads=1          # Run tests sequentially
```

### Development Workflow
Always run after making changes:
```bash
cargo fmt        # Format code
cargo clippy      # Check for lints
cargo test        # Run tests
```

## Code Style Guidelines

### Formatting
- Use `cargo fmt` before committing
- Default to 4-space indentation
- Maximum line width: 100 characters

### Import Organization
- Order: std imports, external crates, internal modules
- Use `use` statements consistently at top of files
- Prefer `crate::` for internal imports over `super::`
- Group related imports together

```rust
use std::collections::HashMap;
use std::fs::File;

use serde::Deserialize;

use crate::module::function;
```

### Naming Conventions
- **Variables/Functions**: `snake_case`
- **Types/Structs/Enums/Traits**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`
- **Lifetime parameters**: Short names like `'a`, `'b`

```rust
fn calculate_sum(a: i32, b: i32) -> i32 { ... }

struct MyStruct { ... }

const MAX_SIZE: usize = 100;

mod my_module { ... }
```

### Type Guidelines
- Explicit types in function signatures
- Use `impl Trait` for simple return types
- Prefer `String` for owned data, `&str` for borrowed
- Use `Vec<T>` for dynamic arrays, `[T; N]` for fixed-size
- Prefer `Box<T>` for heap allocation

### Error Handling
- Use `Result<T, E>` for recoverable errors
- Use `Option<T>` for optional values
- Propagate errors with `?` operator
- Use `anyhow` or `thiserror` for error types (if added)
- Avoid `unwrap()` except in tests or guaranteed cases

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
```

### Code Organization
- One public struct/enum/trait per module (generally)
- Separate `mod` declarations and implementations
- Use `#[cfg(test)]` for test modules
- Keep functions focused and small (< 50 lines)

### Documentation
- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Include examples in doc comments

```rust
/// Calculates the factorial of a number.
///
/// # Examples
///
/// ```
/// assert_eq!(factorial(5), 120);
/// ```
pub fn factorial(n: u32) -> u32 { ... }
```

### Testing Guidelines
- Unit tests in same file with `#[cfg(test)]`
- Integration tests in `tests/` directory
- Use descriptive test names
- Test both success and error cases

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculates_sum_correctly() {
        assert_eq!(calculate_sum(2, 3), 5);
    }
}
```

### Best Practices
- Prefer iterators over loops where appropriate
- Use `match` expressions for exhaustive pattern matching
- Leverage Rust's ownership model (move, borrow, clone)
- Avoid `unsafe` code unless absolutely necessary
- Use `#[allow(...)]` sparingly and document why

## Commit Guidelines
- Format code with `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Ensure all tests pass: `cargo test`
- Write clear, descriptive commit messages
