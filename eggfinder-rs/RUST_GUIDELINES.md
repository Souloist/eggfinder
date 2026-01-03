# Rust Coding Guidelines

Best practices for this codebase. Follow these rules when writing Rust code.

## Error Handling

**Never use `unwrap()` or panic-inducing APIs in production code.**

```rust
// ❌ Bad - panics on None
let value = some_option.unwrap();

// ❌ Bad - panics on Err
let result = fallible_fn().unwrap();

// ✅ Good - handle explicitly
let value = match some_option {
    Some(v) => v,
    None => return Err(MyError::MissingValue),
};

// ✅ Good - propagate with ?
let result = fallible_fn()?;

// ✅ Good - provide default
let value = some_option.unwrap_or_default();
let value = some_option.unwrap_or_else(|| compute_default());
```

**Exception:** `unwrap()`, `expect()`, and `assert!` are acceptable in tests.

## Module Imports

**Prefer `crate::` over `super::` for module imports.**

```rust
// ❌ Avoid
use super::board::Board;
use super::super::constants::GameConfig;

// ✅ Prefer
use crate::board::Board;
use crate::constants::GameConfig;
```

## Re-exports

**Avoid `pub use` unless intentionally re-exporting for public API.**

```rust
// ❌ Avoid re-exporting dependencies
pub use rand::Rng;
pub use serde::Serialize;

// ✅ OK - re-exporting your own types for convenience
pub use crate::board::{Board, BoardError};
pub use crate::game_state::GameState;
```

## Global State

**Avoid global state; pass explicit context structs instead.**

```rust
// ❌ Avoid
lazy_static! {
    static ref CONFIG: Config = Config::load();
}
static INSTANCE: Once = Once::new();

// ✅ Prefer - pass context explicitly
struct AppContext {
    config: Config,
    db: Database,
}

fn process(ctx: &AppContext, data: &Data) -> Result<Output, Error> {
    // use ctx.config, ctx.db
}
```

## Summary

| Rule | Rationale |
|------|-----------|
| No `unwrap()` / `panic!` | Graceful error handling, no surprise crashes |
| Use `crate::` imports | Clearer, more maintainable import paths |
| Avoid `pub use` for deps | Prevents leaking implementation details |
| No global state | Explicit dependencies, easier testing |
