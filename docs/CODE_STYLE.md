# Code Style Guidelines

## Linter

We use Clippy as main linter. Configuration can be found in root `Cargo.toml`, under `[workspace.lints.clippy]`.

## Formatter

We use Rustfmt as main formatter. Configuration can be found in `rustfmt.toml`.

## Other rules

### Conversion to `String`

If possible, `.to_owned()` should be used to convert string literals to `String`.
Example:

```rust
// Bad
"foo".to_string();
String::from("foo");

// Good
"foo".to_owned();
```
