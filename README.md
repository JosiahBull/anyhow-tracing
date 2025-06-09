# Anyhow-Tracing

An extension of the `anyhow` crate that provides named fields on an equivalent
of `anyhow::Error`. Named fields are stored as a `Vec<(&'static str, String)>`
to allow for passing the error object around as an owned instance.

This crate provides macros `anyhow_tracing::anyhow!`, `anyhow_tracing::bail!`,
and `anyhow_tracing::ensure!` to ensure feature compatibility with the `anyhow`
crate and can be used as a drop-in replacement.

⚠️ **Performance Note**: This comes at a significant performance and memory cost
for error handling above and beyond the existing anyhow crate.

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

## Error Methods

The `Error` type provides several methods for working with fields:

```rust
use anyhow_tracing::{anyhow, Error, Context};

fn error_methods_example() {
    let err: Error = anyhow!(field1 = "value1", field2 = "value2", "Test error");

    // Get a specific field
    let field_value = err.get_field("field1"); // Some("value1")

    // Get all fields
    let all_fields = err.fields(); // &[(&'static str, String)]

    // Create new errors by adding fields
    let err2 = anyhow!(field1 = "value1", field2 = "value2", "Test error");
    let err_with_more_fields = err2.with_field("field3", "value3");

    let err3 = anyhow!(field1 = "value1", field2 = "value2", "Test error");
    let err_with_debug_field = err3.with_field_debug("field4", vec![1, 2, 3]);

    // Add context while preserving fields
    let err4 = anyhow!(field1 = "value1", field2 = "value2", "Test error");
    let err_with_context = err4.context("Additional context");
}
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

## Error Display

Errors display both the error message and fields:

```rust
use anyhow_tracing::{anyhow, Error};
let err: Error = anyhow!(
    user_id = "user123",
    session_id = "sess456",
    "Authentication failed"
);

println!("{}", err);
```

## Performance Considerations

This crate adds overhead compared to plain `anyhow`:

- **Memory**: Each error stores a vector of field key-value pairs.
- **Allocation**: Field values are converted to `String` for storage and for ownership to allow arbitrary movement of the error object.

## Compatibility

This crate is designed to be a drop-in replacement for `anyhow` with additional functionality. Most `anyhow` code should work with minimal changes, primarily requiring:

1. Import changes: `anyhow` → `anyhow_tracing`
2. Macro syntax: Use `,` to separate fields and messages - similar to `tracing::event!`.
3. Method chaining: Be explicit about context operations to avoid trait conflicts

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
