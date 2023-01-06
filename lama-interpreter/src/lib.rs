mod builtin;
mod call_stack;
mod error;
mod interpreter;
mod scope;
mod stack;
mod value;
#[cfg(all(target_os = "linux", target_arch = "x86"))]
mod unsafe_builtin;

pub use interpreter::Interpreter;
pub use builtin::RustEnvironment;
#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub use unsafe_builtin::UnsafeEnvironment;