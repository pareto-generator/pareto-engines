// Setters are a bit untypical for rust,
// but we want to have "composeable" struct creation.

use crate::ast::Span;
use crate::common::FromStrAndSpan;
use crate::error::DatamodelError;

/// Trait for all datamodel objects which have a name.
pub trait WithName {
    /// Gets the name.
    fn name(&self) -> &String;
    /// Sets the name.
    fn set_name(&mut self, name: &str);
}

/// Trait for all datamodel objects which have an internal database name.
pub trait WithDatabaseName {
    /// Gets the internal database name.
    fn database_name(&self) -> &Option<String>;
    /// Sets the internal database name.
    fn set_database_name(&mut self, database_name: &Option<String>);
}

pub trait Parsable: Sized {
    fn parse(s: &str) -> Option<Self>;

    fn descriptor() -> &'static str;
}

impl<T> FromStrAndSpan for T
where
    T: Parsable,
{
    fn from_str_and_span(s: &str, span: Span) -> Result<Self, DatamodelError> {
        match T::parse(s) {
            Some(x) => Ok(x),
            None => Err(DatamodelError::new_literal_parser_error(T::descriptor(), s, span)),
        }
    }
}
