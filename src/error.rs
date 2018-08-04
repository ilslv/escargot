use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Spawning the cargo subommand failed.
    InvalidCommand,
    /// The cargo subcommand returned an error.
    CommandFailed,
    /// Parsing the cargo subcommand's output failed.
    InvalidOutput,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::InvalidOutput => write!(f, "Spawning the cargo subommand failed."),
            ErrorKind::CommandFailed => write!(f, "The cargo subcommand returned an error."),
            ErrorKind::InvalidCommand => write!(f, "Parsing the cargo subcommand's output failed."),
        }
    }
}

#[derive(Debug)]
pub struct CargoError {
    kind: ErrorKind,
    context: Option<String>,
    cause: Option<Box<Error + Send + Sync + 'static>>,
}

impl CargoError {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            context: None,
            cause: None,
        }
    }

    pub fn set_context<S>(mut self, context: S) -> Self
    where
        S: Into<String>,
    {
        let context = context.into();
        self.context = Some(context);
        self
    }

    pub fn set_cause<E>(mut self, cause: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let cause = Box::new(cause);
        self.cause = Some(cause);
        self
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl Error for CargoError {
    fn description(&self) -> &str {
        "Cargo command failed."
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|c| {
            let c: &Error = c.as_ref();
            c
        })
    }
}

impl fmt::Display for CargoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Cargo command failed: {}", self.kind)?;
        if let Some(ref context) = self.context {
            writeln!(f, "{}", context)?;
        }
        if let Some(ref cause) = self.cause {
            writeln!(f, "Cause: {}", cause)?;
        }
        Ok(())
    }
}

pub type CargoResult<T> = Result<T, CargoError>;
