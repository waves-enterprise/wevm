mod exec;
mod runtime;
mod stack;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Error {
    /// Failed to parse and validate Wasm bytecode
    InvalidBytecode,
    /// An error that may occur upon operating with virtual or linear memory
    MemoryError,
    /// Limits limit the amount of memory well below u32::MAX
    MemoryLimits,
    /// An error that may occur upon operating with Linker instances
    LinkerError,
    /// Could not find stack
    StackNotFound,
    /// Failed to instantiate and start the Wasm bytecode
    InstantiateFailed,
    /// Failed parse function name
    FailedParseFuncName,
    /// Could not find function
    FuncNotFound,
    /// invalid number of arguments
    InvalidNumArgs,
    /// Failed to parse function argument
    FailedParseFuncArgs,
    /// Failed during execution
    FailedExec,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
