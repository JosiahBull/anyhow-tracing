# Anyhow-Tracing

[![Crates.io](https://img.shields.io/crates/v/anyhow-tracing.svg)](https://crates.io/crates/anyhow-tracing)
[![Documentation](https://docs.rs/anyhow-tracing/badge.svg)](https://docs.rs/anyhow-tracing)
[![Tests](https://github.com/JosiahBull/anyhow-tracing/workflows/Tests/badge.svg)](https://github.com/JosiahBull/anyhow-tracing/actions)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)

An extension of the `anyhow` crate that provides named fields on an equivalent
of `anyhow::Error`. Named fields are stored as a `Vec<(&'static str, Box<str>)>`
to allow for passing the error object around as an owned instance.

## Features

- **Named Fields**: Add structured key-value fields to errors for better debugging and logging.
- **Drop-in Replacement**: Compatible macros that work like `anyhow!`, `bail!`, and `ensure!`.
- **Error Chaining**: Preserves error chains while maintaining named fields, new named fields are added to the error.
- **Debug and Display Fields**: Support for both `Display` and `Debug` formatting of field values.
- **Context Extension**: Extends the `Context` trait to work with named fields.

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
anyhow-tracing = "0.1"
```

## Basic Usage

```rust
use anyhow_tracing::{anyhow, bail, ensure, Context, Error, Result};

// Create errors with named fields
let err: Error = anyhow!(
    user_id = "user123",
    session_id = "sess456",
    "Authentication failed for user"
);

// Access field values
println!("User ID: {:?}", err.get_field("user_id"));
println!("Session ID: {:?}", err.get_field("session_id"));

// Error with debug formatting
let data = vec![1, 2, 3];
let err: Error = anyhow!(data = ?data, "Processing failed");

// Using bail! macro
fn validate_input(value: i32) -> Result<()> {
    if value < 0 {
        bail!(
            input_value = value.to_string(),
            constraint = "non_negative",
            "Invalid input");
    }
    Ok(())
}

// Using ensure! macro
fn process_number(value: i32) -> Result<i32> {
    ensure!(value >= 0, value, "Number must be non-negative");
    ensure!(value <= 100, value, max_allowed = "100", "Number must be at most 100");
    Ok(value * 2)
}

// Error chaining with context
let result: Result<()> = Err(anyhow!(component = "postgresql", "Database error"));
let final_result = match result {
    Ok(v) => Ok(v),
    Err(e) => Err(e.context("User service unavailable"))
};

// Working with standard library errors
use std::fs;
let result = fs::read_to_string("/nonexistent/file")
    .with_field("operation", "read_config")
    .with_field("file_type", "toml");
```

## Context Trait Extensions

The crate extends the `Context` trait to work with standard library types:

```rust
use anyhow_tracing::{Context, Result};
use std::fs::File;

// On Result types
let result: Result<File> = File::open("/nonexistent/path")
    .with_field("operation", "file_open");

// On Option types
let maybe_value: Option<String> = None;
let result: Result<String> = maybe_value
    .with_field("context", "parsing")
    .context("Value was None");
```

## Compatibility

This crate is designed to be a drop-in replacement for `anyhow` with additional functionality. Most `anyhow` code should work with minimal changes, primarily requiring:

1. Import changes: `anyhow` → `anyhow_tracing`
2. Macro syntax: Use `,` to separate fields and messages - similar to `tracing::event!`.
3. Method chaining: Be explicit about context operations to avoid trait conflicts

## 📄 License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
