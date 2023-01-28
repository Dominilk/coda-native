//! This is the coda programming language native crate. Coda is a modern, general purpose programming language.

/// Enum holding all native coda values.
#[derive(Debug, Clone)]
pub enum CodaValue {
    Character(char),
    Long(i128),
    Integer(i32),
    Double(f64),
    Byte(u8),
    Boolean(bool),
    String(String)
}

/// Enum containing all possible control-flow impacts a [NativeBind] could have.
#[derive(Debug, Clone)]
pub enum ControlFlowImpact {
    Return(CodaValue),
    Break,
    Continue
}

/// Struct representing a coda native-bind, offering an interface between
/// coda code and rust-compiled native code.
#[derive(Debug, Clone, Hash)]
pub struct NativeBind {
    /// The name/identifier of the [NativeBind].
    pub name: String,
    /// The function handling the interfacing.
    pub handler: fn(Vec<CodaValue>) -> ControlFlowImpact
}