//! This is the coda programming language native crate. Coda is a modern, general purpose programming language.

use std::fmt;
use std::ffi::OsStr;


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
    pub name: &'static str,
    /// The function handling the interfacing.
    pub handler: fn(Vec<NativeValue>) -> Option<ControlFlowImpact> // TODO: Consider changing to &NativeValue
}

/// Struct representing an error occured when trying to load [NativeBind]s.
#[derive(Debug)]
pub enum BindLoadError {
    Simple(&'static str),
    DlOpen(dlopen::Error)
}

impl std::error::Error for BindLoadError {}

impl From<dlopen::Error> for BindLoadError {
    fn from(error: dlopen::Error) -> Self {
        Self::DlOpen(error)
    }
}

impl fmt::Display for BindLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BindLoadError: ")?;
        
        match self {
            Self::Simple(message) => formatter.write_str(message),
            Self::DlOpen(error) => fmt::Display::fmt(error, formatter)
        }
    }
}

/// Loads all [NativeBind]s contained in the library at the specified path.
pub fn load_binds(name: impl AsRef<OsStr>) -> Result<Vec<NativeBind>, BindLoadError> {
    let library = dlopen::symbor::Library::open(name)?;
    
    Ok(unsafe { library.symbol::<fn() -> Vec<NativeBind>>("bootstrap")? }())
}