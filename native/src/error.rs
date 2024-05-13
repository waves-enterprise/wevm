use jni::sys::jint;

pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Enumeration of errors possible as a result of VM operation.
#[derive(Debug, PartialEq)]
pub enum Error {
    Executable(ExecutableError),
    Jvm(JvmError),
    Runtime(RuntimeError),
}

impl Error {
    /// Converting an error to a numeric code of `jint` type.
    pub fn as_jint(&self) -> jint {
        match self {
            Error::Executable(error) => *error as jint,
            Error::Jvm(error) => *error as jint,
            Error::Runtime(error) => *error as jint,
        }
    }

    /// Converting an error to a numeric code of `i32` type.
    pub fn as_i32(&self) -> i32 {
        match self {
            Error::Executable(error) => *error as i32,
            Error::Jvm(error) => *error as i32,
            Error::Runtime(error) => *error as i32,
        }
    }
}

/// Enumeration of errors that may occur as a result of
/// WASM enigne preparation and bytecode processing.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExecutableError {
    /// Failed to parse and validate Wasm bytecode.
    InvalidBytecode = 100,
    /// Could not found constructor.
    ConstructorNotFound = 101,
    /// An error that may occur upon operating with virtual or linear memory.
    MemoryError = 102,
    /// Limits limit the amount of memory well below u32::MAX.
    MemoryLimits = 103,
    /// An error that may occur upon operating with Linker instances.
    LinkerError = 104,
    /// Failed to instantiate and start the Wasm bytecode.
    InstantiateFailed = 105,
    /// Global heap base not found.
    HeapBaseNotFound = 106,
    /// Could not find function.
    FuncNotFound = 107,
    /// invalid number of arguments.
    InvalidNumArgs = 108,
    /// Failed to parse function argument.
    FailedParseFuncArgs = 109,
    /// Failed to parse (DataEntry arguments or Payments).
    FailedDeserialize = 110,
    /// Failed during execution.
    FailedExec = 111,
    /// Call stack overflow error.
    StackOverflow = 112,
    /// Failed receiving Module.
    ModuleNotFound = 113,
    /// Fuel metering is disabled.
    FuelMeteringDisabled = 114,
}

/// Enumeration of errors that can occur while accessing the JVM.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JvmError {
    /// Failed receiving JVM.
    JvmNotFound = 200,
    /// Failed receiving JVM Callback.
    JvmCallbackNotFound = 201,
    /// Failed attaches the current thread to the Java VM.
    AttachCurrentThread = 202,
    /// Failed JVM method call.
    MethodCall = 203,
    /// Failed byte array conversion.
    ByteArrayConversion = 204,
    /// Failed receiving JavaVM interface.
    GetJavaVM = 205,
    /// Error callback new_global_ref.
    NewGlobalRef = 206,
    /// Couldn't create java byte array.
    NewByteArray = 207,
    /// Couldn't create java string.
    NewString = 208,
    /// Failed to receive object.
    ReceiveObject = 209,
    /// Failed to receive byte.
    ReceiveByte = 210,
    /// Failed to receive integer.
    ReceiveInt = 211,
    /// Failed to receive long.
    ReceiveLong = 212,
    /// Failed to receive boolean
    ReceiveBoolean = 213,
}

/// Enumeration of errors that may occur while working inside the interpreter.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RuntimeError {
    /// Exception thrown during contract execution.
    Exception = 300,
    /// Failed receiving Memory.
    MemoryNotFound = 301,
    /// Failed to retrieve UTF-8 string.
    Utf8Error = 302,
    /// Incorrect result obtained during contract execution.
    InvalidResult = 303,
    /// Error converting Base58 string to bytes.
    Base58Error = 304,
    /// Error when converting numeric types.
    ConvertingNumericTypes = 305,
    /// AssetHolder type not found.
    AssetHolderTypeNotFound = 306,
    /// Address version not found.
    AddressVersionNotFound = 307,
    /// Value parsing error.
    ParseError = 308,
}

#[cfg(test)]
mod tests {
    use super::*;
    use jni::sys::jint;

    #[test]
    fn test_error() {
        assert_eq!(ExecutableError::InvalidBytecode as jint, 100);
        assert_eq!(JvmError::JvmNotFound as jint, 200);
        assert_eq!(RuntimeError::Exception as jint, 300);
    }
}
