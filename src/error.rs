use std::error::Error as StdError;
use std::fmt;

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// An error type that extends `anyhow::Error` with named fields.
pub struct Error {
    /// The underlying anyhow error
    inner: anyhow::Error,
    /// Named fields stored as key-value pairs
    fields: Vec<(&'static str, Box<str>)>,
}

impl Error {
    /// Create a new error from an anyhow error.
    pub const fn new(error: anyhow::Error) -> Self {
        Self {
            inner: error,
            fields: Vec::new(),
        }
    }

    /// Create a new error with a message and optional fields.
    pub fn msg<T: fmt::Display + fmt::Debug + Send + Sync + 'static>(msg: T) -> Self {
        Self::new(anyhow::Error::msg(msg))
    }

    /// Add a named field to this error.
    pub fn with_field<V: fmt::Display>(mut self, key: &'static str, value: V) -> Self {
        self.fields.push((key, value.to_string().into_boxed_str()));
        self
    }

    /// Add a named field with debug formatting to this error.
    pub fn with_field_debug<V: fmt::Debug>(mut self, key: &'static str, value: V) -> Self {
        self.fields
            .push((key, format!("{:?}", value).into_boxed_str()));
        self
    }

    /// Get the named fields.
    pub fn fields(&self) -> &[(&'static str, Box<str>)] {
        &self.fields
    }

    /// Get a specific field value by key, this is an O(n) operation.
    pub fn get_field(&self, key: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.as_ref())
    }

    /// Add context to this error, see [`anyhow::Context`] for more details.
    pub fn context<C: fmt::Display + Send + Sync + 'static>(self, context: C) -> Self {
        Self {
            inner: self.inner.context(context),
            fields: self.fields,
        }
    }

    /// Add context to this error with a closure, see [`anyhow::Context`] for more details.
    pub fn with_context<C, F>(self, f: F) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        Self {
            inner: self.inner.context(f()),
            fields: self.fields,
        }
    }

    /// Get the root cause of this error.
    pub fn root_cause(&self) -> &dyn StdError {
        self.inner.root_cause()
    }

    /// Get the chain of errors.
    pub fn chain(&self) -> anyhow::Chain {
        self.inner.chain()
    }

    /// Downcast the error to a concrete type.
    pub fn downcast<E>(self) -> std::result::Result<E, Self>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        match self.inner.downcast::<E>() {
            Ok(e) => Ok(e),
            Err(inner) => Err(Self {
                inner,
                fields: self.fields,
            }),
        }
    }

    /// Downcast the error to a reference to a concrete type.
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast_ref::<E>()
    }

    /// Downcast the error to a mutable reference to a concrete type.
    pub fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast_mut::<E>()
    }

    /// Check if the error is of a particular type.
    pub fn is<E>(&self) -> bool
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.is::<E>()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display the main error
        write!(f, "{}", self.inner)?;

        // Add fields if any
        if !self.fields.is_empty() {
            write!(f, " [")?;
            for (i, (key, value)) in self.fields.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}={}", key, value)?;
            }
            write!(f, "]")?;
        }

        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display the main error
        write!(f, "{:?}", self.inner)?;

        // Add fields if any
        if !self.fields.is_empty() {
            write!(f, "\n\nFields:\n")?;
            for (i, (key, value)) in self.fields.iter().enumerate() {
                write!(f, "\t{}: {:?}", key, value)?;
                if i < self.fields.len().saturating_sub(1) {
                    write!(f, ",")?;
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source()
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self::new(error)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::msg(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Self::msg(msg.to_string())
    }
}

/// Extension trait for adding context and fields to errors.
pub trait Context<T> {
    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static;

    /// Wrap the error value with additional context that is evaluated lazily.
    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
        F: FnOnce() -> C;

    /// Add a named field to the error.
    fn with_field<V>(self, key: &'static str, value: V) -> Result<T>
    where
        V: fmt::Display;

    /// Add a named field with debug formatting to the error.
    fn with_field_debug<V>(self, key: &'static str, value: V) -> Result<T>
    where
        V: fmt::Debug;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.map_err(|e| Error::from(anyhow::Error::from(e)).context(context))
    }

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| Error::from(anyhow::Error::from(e)).with_context(f))
    }

    fn with_field<V>(self, key: &'static str, value: V) -> Result<T>
    where
        V: fmt::Display,
    {
        self.map_err(|e| Error::from(anyhow::Error::from(e)).with_field(key, value))
    }

    fn with_field_debug<V>(self, key: &'static str, value: V) -> Result<T>
    where
        V: fmt::Debug,
    {
        self.map_err(|e| Error::from(anyhow::Error::from(e)).with_field_debug(key, value))
    }
}

impl<T> Context<T> for Option<T> {
    fn context<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.ok_or_else(|| Error::msg(context))
    }

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.ok_or_else(|| Error::msg(f()))
    }

    fn with_field<V>(self, key: &'static str, value: V) -> Result<T>
    where
        V: fmt::Display,
    {
        self.ok_or_else(|| Error::msg("None value").with_field(key, value))
    }

    fn with_field_debug<V>(self, key: &'static str, value: V) -> Result<T>
    where
        V: fmt::Debug,
    {
        self.ok_or_else(|| Error::msg("None value").with_field_debug(key, value))
    }
}
