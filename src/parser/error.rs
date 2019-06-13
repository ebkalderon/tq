//! Errors thrown when parsing filters and modules.

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::iter;

use colored::*;
use pom::Error as PomError;

/// An error thrown while parsing a filter.
#[derive(Clone, Debug)]
pub struct FilterError {
    inner: PomError,
    filter: String,
}

impl FilterError {
    /// Creates a new `FilterError` from the given `pom::Error` and filter string.
    pub fn new<T: ToString>(inner: PomError, filter: T) -> Self {
        FilterError {
            inner,
            filter: filter.to_string(),
        }
    }
}

impl Display for FilterError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let (msg, pos, extra) = self.inner.to_display_data();

        let error = format!("{}{}{}", "filter error".red(), ": ".white(), msg.white()).bold();

        let pos = pos.unwrap_or_else(|| self.filter.len());
        let pointer: String = iter::repeat("_").take(pos).chain(iter::once("^")).collect();
        let filter = format!("{}\n  {}", self.filter.white(), pointer.red().bold());

        let extra = extra
            .map(|e| format!("\n\nadditional details: {}", e))
            .unwrap_or_default();

        write!(fmt, "{}\n\n  {}{}", error, filter, extra)
    }
}

impl Error for FilterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner as _)
    }
}

/// Extension trait for `pom::Error`.
trait PomErrorExt {
    /// Returns the error as a tuple of the following members:
    ///
    /// * String representation of the error.
    /// * Character index where the error occurred, if known.
    /// * Additional error information, if present.
    fn to_display_data(&self) -> (String, Option<usize>, Option<Box<PomError>>);
}

impl PomErrorExt for PomError {
    fn to_display_data(&self) -> (String, Option<usize>, Option<Box<PomError>>) {
        match self {
            PomError::Incomplete => {
                let message = "input is incomplete, expected final expression".to_owned();
                (message, None, None)
            }
            PomError::Mismatch { message, position } => {
                let message = message.clone();
                (message, Some(*position), None)
            }
            PomError::Conversion { message, position } => {
                let message = message.clone();
                (message, Some(*position), None)
            }
            PomError::Expect {
                message,
                position,
                inner,
            } => {
                let message = message.clone();
                let extra = inner.clone();
                (message, Some(*position), Some(extra))
            }
            PomError::Custom {
                message,
                position,
                inner,
            } => {
                let message = message.clone();
                let extra = inner.clone();
                (message, Some(*position), extra)
            }
        }
    }
}
