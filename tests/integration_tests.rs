#![allow(clippy::tests_outside_test_module, reason = "integration tests")]

use std::io;

use anyhow_tracing::{Context, Error, Result, anyhow, bail, ensure};
use insta::assert_snapshot;

/// Tests the various forms of the `anyhow!` macro for creating errors.
/// This single test covers creating errors with:
///
/// - A simple string literal message.
/// - A formatted message.
/// - Implicitly named fields from variables (`value`).
/// - Explicitly named fields with Display formatting (`operation`).
/// - Explicitly named fields with Debug formatting (`debug_data`).
///
/// It ensures all fields are correctly captured and the error message is formatted.
#[test]
fn test_anyhow_macro_with_various_fields() {
    let value = 42;
    let debug_data = vec!["a", "b"];

    let err: Error = anyhow!(
        value, // implicit name from variable
        operation = %"login", // named field with Display
        debug_data = ?debug_data, // named field with Debug
        "User '{}' failed to log in",
        "alice"
    );

    // Check the formatted error message
    assert!(err.to_string().contains("User 'alice' failed to log in"));

    // Check that all fields were captured correctly
    assert_eq!(err.fields().len(), 3);
    assert_eq!(err.get_field("value"), Some("42"));
    assert_eq!(err.get_field("operation"), Some("login"));
    assert_eq!(err.get_field("debug_data"), Some(r#"["a", "b"]"#));

    // Snapshot the error display and debug representations
    assert_snapshot!("anyhow_macro_error_display", format!("{}", err));
    assert_snapshot!("anyhow_macro_error_debug", format!("{:?}", err));

    // Check that context can be added and fields are preserved
    let contextual_err = err.context("additional context");
    assert!(contextual_err.to_string().contains("additional context"));
    assert_eq!(contextual_err.get_field("value"), Some("42"));

    // Snapshot the contextual error
    assert_snapshot!(
        "anyhow_macro_contextual_error_display",
        format!("{}", contextual_err)
    );
    assert_snapshot!(
        "anyhow_macro_contextual_error_debug",
        format!("{:?}", contextual_err)
    );
}

/// Tests that the `bail!` macro creates and returns an error immediately.
/// It verifies that `bail!` accepts the same field and message formatting
/// arguments as `anyhow!` and that the function returns `Err`.
#[test]
fn test_bail_macro_creates_and_returns_error() {
    #[allow(unreachable_code, reason = "tests")]
    fn inner_function() -> Result<()> {
        let user_id = "user-123";
        bail!(?user_id, attempt = %3, "Authentication failed");
        panic!("This should not be reached");
    }

    // Call the function and assert it returns an error
    let result = inner_function();
    assert!(result.is_err());

    // Inspect the error to ensure it was created correctly
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Authentication failed"));
    assert_eq!(err.fields().len(), 2);
    assert_eq!(err.get_field("user_id"), Some(r#""user-123""#));
    assert_eq!(err.get_field("attempt"), Some("3"));

    // Snapshot the bail error
    assert_snapshot!("bail_macro_error_display", format!("{}", err));
    assert_snapshot!("bail_macro_error_debug", format!("{:?}", err));
}

/// Tests that the `ensure!` macro returns `Ok` on a true condition
/// and returns `Err` on a false condition.
/// It also verifies that the error created on failure contains the
/// specified message and structured fields.
#[test]
fn test_ensure_macro_handles_conditions_correctly() {
    fn check_password(is_valid: bool) -> Result<()> {
        ensure!(is_valid, policy = %"8+ chars", "Password is too short");
        if !is_valid {
            panic!("This should not be reached");
        }
        Ok(())
    }

    // Test the success case where the condition is true
    check_password(true).unwrap();

    // Test the failure case where the condition is false
    let result = check_password(false);
    assert!(result.is_err());

    // Inspect the error from the failure case
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Password is too short"));
    assert_eq!(err.fields().len(), 1);
    assert_eq!(err.get_field("policy"), Some("8+ chars"));

    // Snapshot the ensure error
    assert_snapshot!("ensure_macro_error_display", format!("{}", err));
    assert_snapshot!("ensure_macro_error_debug", format!("{:?}", err));
}

/// Tests the `Context` trait implementation for `Result` and `Option`.
/// This verifies that `.context()` and `.with_field()` can be called on
/// `Result::Err` and `Option::None` to produce a rich `anyhow_tracing::Error`.
#[test]
fn test_context_trait_on_result_and_option() {
    // 1. Test context on a standard library `Result::Err`
    let io_result: std::result::Result<(), io::Error> =
        Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));

    let err = io_result
        .context("File operation failed")
        .with_field("filename", "config.toml")
        .unwrap_err();

    assert_eq!(
        err.to_string(),
        "File operation failed [filename=config.toml]"
    );

    // Snapshot the result context error
    assert_snapshot!("context_result_error_display", format!("{}", err));
    assert_snapshot!("context_result_error_debug", format!("{:?}", err));

    // 2. Test context on an `Option::None`
    let none_val: Option<i32> = None;
    let err = none_val
        .context("Value was missing")
        .with_field("variable_name", "port")
        .unwrap_err();

    assert!(err.to_string().contains("Value was missing"));
    assert_eq!(err.get_field("variable_name"), Some("port"));

    // Snapshot the option context error
    assert_snapshot!("context_option_error_display", format!("{}", err));
    assert_snapshot!("context_option_error_debug", format!("{:?}", err));
}

/// Tests the ability to wrap a standard error, add fields, and then
/// downcast it back to its original type.
#[test]
fn test_error_wrapping_and_downcasting() {
    let original_error = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");

    // Wrap the original error and add context
    let err: Error =
        Error::from(anyhow::Error::from(original_error)).with_field("request_id", "req-abc");

    // Verify that the field was added
    assert_eq!(err.get_field("request_id"), Some("req-abc"));

    // Use `is()` to check for the underlying error type
    assert!(err.is::<io::Error>());
    assert!(!err.is::<std::fmt::Error>());

    // Use `downcast_ref()` to get a reference to the original error
    let downcast_ref = err.downcast_ref::<io::Error>();
    assert!(downcast_ref.is_some());
    assert_eq!(
        downcast_ref.unwrap().kind(),
        io::ErrorKind::PermissionDenied
    );

    // Snapshot the wrapped error
    assert_snapshot!("wrapped_error_display", format!("{}", err));
    assert_snapshot!("wrapped_error_debug", format!("{:?}", err));
}

/// Tests that the macros and methods correctly handle various edge cases for field values.
#[test]
fn test_field_value_edge_cases() {
    let special_val = r#"value with "quotes" and \backslashes"#;
    let long_val = "a".repeat(1000);

    let err: Error = anyhow!(
        empty_field = %"",
        whitespace_field = %"   ",
        special_field = %special_val,
        unicode_field = %"测试",
        long_field = %&long_val,
        "testing edge cases"
    );

    assert_eq!(err.get_field("empty_field"), Some(""));
    assert_eq!(err.get_field("whitespace_field"), Some("   "));
    assert_eq!(err.get_field("special_field"), Some(special_val));
    assert_eq!(err.get_field("unicode_field"), Some("测试"));
    assert_eq!(err.get_field("long_field").unwrap().len(), 1000);

    // Snapshot the edge cases error
    assert_snapshot!("edge_cases_error_display", format!("{}", err));
    assert_snapshot!("edge_cases_error_debug", format!("{:?}", err));
}

/// Tests methods on the `Error` type for manipulating context and fields.
#[test]
fn test_error_methods_for_context_and_fields() {
    let err = anyhow!("base error")
        // Add a field with a display value
        .with_field("field1", "value1")
        // Add a field with a debug value
        .with_field_debug("field2", vec![10, 20])
        // Add context with an eagerly evaluated string
        .context("eager context")
        // Add context with a lazily evaluated string
        .with_context(|| "lazy context");

    // Check error message for all context
    let msg = err.to_string();
    assert_eq!(msg, "lazy context [field1=value1, field2=[10, 20]]");

    // Check for all fields
    assert_eq!(err.get_field("field1"), Some("value1"));
    assert_eq!(err.get_field("field2"), Some("[10, 20]"));

    // Check the error chain
    let chain: Vec<String> = err.chain().map(|e| e.to_string()).collect();
    assert_eq!(chain.len(), 3); // lazy context, eager context, base error
    assert_eq!(chain[0], "lazy context");
    assert_eq!(chain[1], "eager context");
    assert_eq!(chain[2], "base error");

    // Snapshot the complex error with multiple contexts and fields
    assert_snapshot!("complex_error_display", format!("{}", err));
    assert_snapshot!("complex_error_debug", format!("{:?}", err));
}
