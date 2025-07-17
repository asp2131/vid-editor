//! Core library crate for VideoAI.
//! For now we expose minimal video I/O structs so the CLI can compile.

pub mod input;
pub mod output;

pub use input::VideoInput;
pub use output::VideoOutput;
