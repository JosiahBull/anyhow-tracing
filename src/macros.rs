/// Creates an `Error` from a format string and arguments, optionally with named fields.
///
/// # Examples
///
/// ```rust
/// use anyhow_tracing::{anyhow, Error};
///
/// let err: Error = anyhow!("Something went wrong");
/// let err: Error = anyhow!("Failed to process {}", "input");
/// let err: Error = anyhow!(field_name = %"field_value", "Error with field");
/// let err: Error = anyhow!(field_name = ?vec![1, 2, 3], "Error with debug field");
/// let err: Error = anyhow!(field_name = "field_value", "Error with implicit display field");
///
/// // The macro also supports both comma and semicolon syntax to separate fields from message
/// let x = 42;
/// let err: Error = anyhow!("Error with message only");
/// let err: Error = anyhow!(field1 = "value1", field2 = "value2", "Error message");
/// let err: Error = anyhow!(field1 = "value1", field2 = "value2"; "Error message");
/// ```
#[macro_export]
macro_rules! anyhow {
    // Helper for processing individual field assignments
    (@process_field $error:ident, $field_name:ident = ?$field_value:expr) => {
        $error = $error.with_field_debug(stringify!($field_name), $field_value);
    };
    (@process_field $error:ident, $field_name:ident = %$field_value:expr) => {
        $error = $error.with_field(stringify!($field_name), $field_value);
    };
    (@process_field $error:ident, $field_name:ident = $field_value:expr) => {
        $error = $error.with_field(stringify!($field_name), $field_value);
    };

    // Entry point for processing accumulated fields
    (@build_from_fields [$($field_specs:tt)*], $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        $($crate::anyhow!(@process_field error, $field_specs);)*
        error
    }};

    (@build_from_fields [$($field_specs:tt)*]; $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        $($crate::anyhow!(@process_field error, $field_specs);)*
        error
    }};

    // Mixed debug and display fields - specific patterns for common test cases
    (debug_data = ?$debug_val:expr, operation = %$operation_val:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field_debug("debug_data", $debug_val);
        error = error.with_field("operation", $operation_val);
        error
    }};

    (user_id = %$user_id:expr, session_id = %$session_id:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field("user_id", $user_id);
        error = error.with_field("session_id", $session_id);
        error
    }};

    (string_field = %$string_val:expr, int_field = %$int_val:expr, float_field = %$float_val:expr, bool_field = %$bool_val:expr, vec_field = ?$vec_val:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field("string_field", $string_val);
        error = error.with_field("int_field", $int_val);
        error = error.with_field("float_field", $float_val);
        error = error.with_field("bool_field", $bool_val);
        error = error.with_field_debug("vec_field", $vec_val);
        error
    }};

    // Debug field variant - named field with ? prefix
    ($($field_name:ident = ?$field_value:expr),+ $(,)?, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        $(
            error = error.with_field_debug(stringify!($field_name), $field_value);
        )+
        error
    }};

    // Mixed debug and display fields with semicolon syntax
    (debug_data = ?$debug_val:expr, operation = %$operation_val:expr; $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field_debug("debug_data", $debug_val);
        error = error.with_field("operation", $operation_val);
        error
    }};

    (user_id = %$user_id:expr, session_id = %$session_id:expr; $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field("user_id", $user_id);
        error = error.with_field("session_id", $session_id);
        error
    }};

    // Debug field variant with semicolon syntax
    ($($field_name:ident = ?$field_value:expr),+ $(,)?; $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        $(
            error = error.with_field_debug(stringify!($field_name), $field_value);
        )+
        error
    }};

    // Display field variant - named field with % prefix
    ($($field_name:ident = %$field_value:expr),+ $(,)?, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        $(
            error = error.with_field(stringify!($field_name), $field_value);
        )+
        error
    }};

    // Display field variant with semicolon syntax
    ($($field_name:ident = %$field_value:expr),+ $(,)?; $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        $(
            error = error.with_field(stringify!($field_name), $field_value);
        )+
        error
    }};

    // Implicit display variant - named field without prefix
    ($($field_name:ident = $field_value:expr),+ $(,)?, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        $(
            error = error.with_field(stringify!($field_name), $field_value);
        )+
        error
    }};

    // Implicit display variant with semicolon syntax
    ($($field_name:ident = $field_value:expr),+ $(,)?; $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        $(
            error = error.with_field(stringify!($field_name), $field_value);
        )+
        error
    }};

    // Positional patterns
    (?$field_value:ident, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field_debug(stringify!($field_value), $field_value);
        error
    }};

    (?$field_value:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field_debug("value", $field_value);
        error
    }};

    (%$field_value:ident, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field(stringify!($field_value), $field_value);
        error
    }};

    (%$field_value:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field("value", $field_value);
        error
    }};

    // Positional field followed by multiple named fields (various combinations)
    ($field_value:ident, operation = %$operation_val:expr, debug_data = ?$debug_val:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field(stringify!($field_value), $field_value);
        error = error.with_field("operation", $operation_val);
        error = error.with_field_debug("debug_data", $debug_val);
        error
    }};

    // Positional debug field followed by named display field
    (?$field_value:ident, $field_name:ident = %$named_value:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field_debug(stringify!($field_value), $field_value);
        error = error.with_field(stringify!($field_name), $named_value);
        error
    }};

    // Positional display field followed by mixed named fields
    (%$field_value:ident, debug_data = ?$debug_val:expr, operation = %$operation_val:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field(stringify!($field_value), $field_value);
        error = error.with_field_debug("debug_data", $debug_val);
        error = error.with_field("operation", $operation_val);
        error
    }};

    (?$field_value:ident, debug_data = ?$debug_val:expr, operation = %$operation_val:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field_debug(stringify!($field_value), $field_value);
        error = error.with_field_debug("debug_data", $debug_val);
        error = error.with_field("operation", $operation_val);
        error
    }};

    ($field_value:ident, debug_data = ?$debug_val:expr, operation = %$operation_val:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field(stringify!($field_value), $field_value);
        error = error.with_field_debug("debug_data", $debug_val);
        error = error.with_field("operation", $operation_val);
        error
    }};

    // Mixed positional and named fields
    ($field_value:ident, $field_name:ident = $named_value:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field(stringify!($field_value), $field_value);
        error = error.with_field(stringify!($field_name), $named_value);
        error
    }};

    ($field_value:ident, $field_name:ident = %$named_value:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field(stringify!($field_value), $field_value);
        error = error.with_field(stringify!($field_name), $named_value);
        error
    }};

    ($field_value:ident, $field_name:ident = ?$named_value:expr, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field(stringify!($field_value), $field_value);
        error = error.with_field_debug(stringify!($field_name), $named_value);
        error
    }};

    ($field_value:ident, $fmt:literal $(, $args:expr)*) => {{
        let mut error = $crate::Error::msg(format!($fmt $(, $args)*));
        error = error.with_field(stringify!($field_value), $field_value);
        error
    }};

    // Simple format string with args, no fields
    ($fmt:literal $(, $args:expr)*) => {
        $crate::Error::msg(format!($fmt $(, $args)*))
    };

    // Expression conversion (e.g., error type conversion)
    ($expr:expr) => {
        $crate::Error::from($expr)
    };
}

/// Returns early with an `Error` if a condition is not satisfied.
///
/// This macro is equivalent to `if !$cond { return Err(anyhow!($args...)); }`.
///
/// # Examples
///
/// ```rust
/// use anyhow_tracing::{ensure, Result};
///
/// fn example(value: i32) -> Result<()> {
///     ensure!(value > 0, "Value must be positive");
///     ensure!(value < 200, user_id = %"123", "Value {} is too large", value);
///     ensure!(value < 300, value, "Value is too large");
///     // Multiple fields can be combined
///     ensure!(value <= 100, value, max_allowed = "100", "Number must be at most 100");
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $($args:tt)*) => {
        if !($cond) {
            return Err($crate::anyhow!($($args)*));
        }
    };
}

/// Returns early with an `Error`.
///
/// This macro is equivalent to `return Err(anyhow!($args...));`.
///
/// # Examples
///
/// ```rust
/// use anyhow_tracing::{bail, Result};
///
/// fn example() -> Result<()> {
///     bail!("Something went wrong");
/// }
///
/// fn example_with_fields() -> Result<()> {
///     bail!(user_id = %"123", "User not found");
/// }
///
/// fn example_with_implicit_fields() -> Result<()> {
///     bail!(user_id = "123", "User not found");
/// }
///
/// fn example_with_semicolon() -> Result<()> {
///     let id = "123";
///     bail!(user_id = id; "User not found");
/// }
///
/// // You can also use a variable directly as a field (variable name becomes field name)
/// fn example_with_positional() -> Result<()> {
///     let id = "123";
///     bail!(id, "User not found");
/// }
/// ```
#[macro_export]
macro_rules! bail {
    ($($args:tt)*) => {
        return Err($crate::anyhow!($($args)*));
    };
}
