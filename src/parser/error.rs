//! Errors thrown when parsing filters and modules.

use std::borrow::Cow;
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
        let (msg, pos, details) = self.inner.to_display_data();

        let error = format!("{}{}{}", "filter error".red(), ": ".white(), msg.white()).bold();

        let pos = pos.unwrap_or_else(|| self.filter.len() - 1);
        println!("error occurred at: {}", pos);
        let pointer: String = iter::repeat("_").take(pos).chain(iter::once("^")).collect();
        let filter = format!("  {}\n  {}", self.filter.white(), pointer.red().bold());

        let details = details
            .map(|e| format!("\n\n{}{}{}", "details".red().bold(), ": ".white().bold(), e))
            .unwrap_or_default();

        write!(fmt, "{}\n\n{}{}", error, filter, details)
    }
}

impl Error for FilterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner as _)
    }
}

/// An error thrown while parsing a filter.
#[derive(Clone, Debug)]
pub struct ModuleError {
    inner: PomError,
    module: String,
}

impl ModuleError {
    /// Creates a new `FilterError` from the given `pom::Error` and filter string.
    pub fn new<T: ToString>(inner: PomError, module: T) -> Self {
        ModuleError {
            inner,
            module: module.to_string(),
        }
    }

    /// Locates the specific line in the module pointed to by the character index and returns a
    /// tuple of the following data:
    ///
    /// * Line number in the module code which contains the given char position.
    /// * A new index pointing to the erroneous character in the line.
    /// * String representation of the line itself.
    fn find_line_containing_index(&self, char_index: usize) -> (usize, usize, String) {
        println!("error occurred at: {}", char_index);

        let line_num = self
            .module
            .bytes()
            .take(char_index + 1)
            .filter(|b| *b == b'\n')
            .count();

        let len_of_prev_lines = self
            .module
            .lines()
            .take(line_num.saturating_sub(1))
            .fold(1, |index, line| index + line.len() + 1);
        let new_index = char_index.saturating_sub(len_of_prev_lines);

        let line = self
            .module
            .lines()
            .nth(line_num.saturating_sub(1))
            .map(ToString::to_string)
            .unwrap_or_default();

        (line_num, new_index, line)
    }
}

impl Display for ModuleError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let (msg, pos, details) = self.inner.to_display_data();

        let error = format!("{}{}{}", "module error".red(), ": ".white(), msg.white()).bold();

        let pos = pos.unwrap_or_else(|| self.module.len() - 1);
        let (line_num, pos, line) = self.find_line_containing_index(pos);
        let pointer: String = iter::repeat("_").take(pos).chain(iter::once("^")).collect();
        let module = format!(
            "{:>4} {} {}\n{:>4}   {}",
            line_num.to_string().blue().bold(),
            "|".blue().bold(),
            line.white(),
            " ",
            pointer.red().bold()
        );

        let details = details
            .map(|e| format!("\n\n{}{}{}", "details".red().bold(), ": ".white().bold(), e))
            .unwrap_or_default();

        write!(fmt, "{}\n\n{}{}", error, module, details)
    }
}

impl Error for ModuleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner as _)
    }
}

/// Extension trait for `pom::Error`.
trait PomErrorExt<'a> {
    /// Returns the error as a tuple of the following members:
    ///
    /// * String representation of the error.
    /// * Character index where the error occurred, if known.
    /// * Additional error information, if present.
    fn to_display_data(&'a self) -> (Cow<'a, str>, Option<usize>, Option<&'a PomError>);
}

impl<'a> PomErrorExt<'a> for PomError {
    fn to_display_data(&'a self) -> (Cow<'a, str>, Option<usize>, Option<&'a PomError>) {
        match self {
            PomError::Incomplete => {
                let message = "input is incomplete, expected final expression".into();
                (message, None, None)
            }
            PomError::Mismatch { message, position } => (message.into(), Some(*position), None),
            PomError::Conversion { message, position } => (message.into(), Some(*position), None),
            PomError::Expect {
                message,
                position,
                inner,
            } => (message.into(), Some(*position), Some(inner as _)),
            PomError::Custom {
                message,
                position,
                inner,
            } => (
                message.into(),
                Some(*position),
                inner.as_ref().map(AsRef::as_ref),
            ),
        }
    }
}
