#![allow(unused)]
// * import modul
pub mod primitive_types;
pub mod collections;
pub mod pointer;
pub mod _control_flow;
pub mod _enums;
pub mod _struct;
pub mod _functions;
pub mod _result;

// import item dari peti atau modul
pub use primitive_types::*;
pub use collections::*;
pub use pointer::*;
pub use _control_flow::*;
pub use _functions::*;
pub use _struct::ex_struct;
pub use _result::*;
