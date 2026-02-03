# Contributing to Wolia

Thank you for your interest in contributing to Wolia!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/wolia.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Run lints: `cargo fmt && cargo clippy`
7. Commit your changes: `git commit -m "Add your feature"`
8. Push to your fork: `git push origin feature/your-feature`
9. Open a Pull Request

## Development Guidelines

### Code Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust naming conventions
- Write documentation for public APIs
- Add tests for new functionality

### Commit Messages

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters
- Reference issues and PRs where appropriate

### Pull Requests

- Keep PRs focused on a single feature or fix
- Update documentation as needed
- Add tests for new functionality
- Ensure all tests pass
- Update CHANGELOG if applicable

## Architecture

### Engine Crates

The engine crates in `/engine` are the shared foundation:

- **wolia-core**: Document model, text, styles
- **wolia-layout**: Text wrapping, pagination
- **wolia-render**: GPU rendering with wgpu
- **wolia-edit**: Cursor, selection, undo/redo
- **wolia-format**: File format handling
- **wolia-platform**: OS integration
- **wolia-math**: Geometry utilities
- **wolia-assets**: Font/image loading
- **wolia-plugin**: Plugin system

### Apps

Apps in `/apps` are thin wrappers that combine:

- Engine crates
- App-specific UI
- App-specific tools

### Engine Modules

Engine modules in `/engine-modules` provide app-specific functionality:

- **grid-engine**: Spreadsheet cells, formulas
- **deck-engine**: Slides, animations

## Questions?

Feel free to open an issue for questions or discussion.
