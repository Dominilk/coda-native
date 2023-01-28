//! This is the coda programming language native crate. Coda is a modern, general purpose programming language.

use std::fmt;

/// Enum holding all native coda values.
#[derive(Debug, Clone, PartialEq)]
pub enum NativeValue {
    Character(char),
    Long(i128),
    Integer(i32),
    Double(f64),
    Byte(u8),
    Boolean(bool),
    String(String)
}

impl fmt::Display for NativeValue {

    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Character(inner) => inner as &dyn fmt::Display,
            Self::Boolean(inner) => inner as &dyn fmt::Display,
            Self::Integer(inner) => inner as &dyn fmt::Display,
            Self::Byte(inner) => inner as &dyn fmt::Display,
            Self::Double(inner) => inner as &dyn fmt::Display,
            Self::Long(inner) => inner as &dyn fmt::Display,
            Self::String(inner) => inner as &dyn fmt::Display,
        }.fmt(formatter)
    }

}

/// Enum containing all possible control-flow impacts a [NativeBind] could have.
#[derive(Debug, Clone, PartialEq)]
pub enum ControlFlowImpact {
    Return(NativeValue),
    Break,
    Continue
}

/// Struct representing a coda native-bind, offering an interface between
/// coda code and rust-compiled native code.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NativeBind {
    /// The name/identifier of the [NativeBind].
    pub name: String,
    /// The function handling the interfacing.
    pub handler: fn(Vec<NativeValue>) -> Option<ControlFlowImpact>
}