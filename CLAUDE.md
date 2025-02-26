# johnnyd - Development Guide

## Build & Test Commands
- Build project: `cargo build`
- Run binary: `cargo run -p johnnyd-bin`
- Run with release optimizations: `cargo run --release -p johnnyd-bin`
- Check for errors without building: `cargo check`
- Lint code: `cargo clippy`
- Run all tests: `cargo test`
- Run specific test: `cargo test test_name`
- Run tests for specific crate: `cargo test -p johnnyd-lib`
- Format code: `cargo fmt`

## Code Style Guidelines
- **Formatting**: Use `cargo fmt` with default settings
- **Linting**: Follow `clippy` recommendations
- **Naming**: snake_case for variables/functions, UpperCamelCase for types/traits
- **Error Handling**: Use Result<T, E> with meaningful error types
- **Documentation**: Use rustdoc-style comments (///) for all public items
- **Testing**: Write tests in a `tests` module or separate test files
- **Modules**: Organize code into modules with clear responsibilities
- **Dependencies**: Keep external dependencies minimal and well-justified
- **Tracing**: Always fully qualify tracing macros (e.g., `tracing::info!`, not `info!`)

## Commit Message Guidelines
- Use single line messages with imperative style
- Prefix commit messages with a single character:
  - `+` for adding features or files
  - `-` for removing features or files
  - `:` for refactoring and cleanups
  - `!` for bugfixes
- Examples:
  - `+ Add JohnnyD struct`
  - `- Remove unused helper functions`
  - `: Refactor error handling`
  - `! Fix incorrect path handling`