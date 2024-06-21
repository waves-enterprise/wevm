use jni::sys::jint;

pub type Result<T, E = Error> = core::result::Result<T, E>;

macro_rules! error {
    (
        enum $name:ident {
            $(
                #[error($message:tt)]
                #[code($code:tt)]
                $variant:ident $( ($value:ident: $_type:ty) )?,
            )+
        }
    ) => {
        #[derive(Debug, PartialEq)]
        pub enum $name {
            $($variant $( ($_type) )? ,)+
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    $($name::$variant $( ($value) )? => {
                        write!(f, "Code: {}. Error: {}.", $code, $message)?;
                        $( write!(f, " Reason: {}.", $value)?; )?
                        Ok(())
                    },)+
                }
            }
        }

        impl $name {
            pub fn as_i32(&self) -> i32 {
                match self {
                    $($name::$variant $( ($value) )? => $code,)+
                }
            }

            pub fn as_jint(&self) -> jint {
                match self {
                    $($name::$variant $( ($value) )? => $code,)+
                }
            }
        }
    }
}

/// Enumeration of errors possible as a result of VM operation.
#[derive(Debug, PartialEq)]
pub enum Error {
    Executable(ExecutableError),
    Jvm(JvmError),
    Runtime(RuntimeError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Executable(error) => write!(f, "{}", error),
            Error::Jvm(error) => write!(f, "{}", error),
            Error::Runtime(error) => write!(f, "{}", error),
        }
    }
}

impl Error {
    /// Converting an error to a numeric code of `jint` type.
    pub fn as_jint(&self) -> jint {
        match self {
            Error::Executable(error) => error.as_jint(),
            Error::Jvm(error) => error.as_jint(),
            Error::Runtime(error) => error.as_jint(),
        }
    }

    /// Converting an error to a numeric code of `i32` type.
    pub fn as_i32(&self) -> i32 {
        match self {
            Error::Executable(error) => error.as_i32(),
            Error::Jvm(error) => error.as_i32(),
            Error::Runtime(error) => error.as_i32(),
        }
    }
}

// Enumeration of errors that may occur as a result of
// WASM enigne preparation and bytecode processing.
error! {
    enum ExecutableError {
        #[error("Failed to parse and validate Wasm bytecode")]
        #[code(100)]
        InvalidBytecode(_message: String),
        #[error("Could not found constructor")]
        #[code(101)]
        ConstructorNotFound,
        #[error("An error that may occur upon operating with virtual or linear memory")]
        #[code(102)]
        MemoryError(_message: String),
        #[error("Limits limit the amount of memory well below u32::MAX")]
        #[code(103)]
        MemoryLimits(_message: String),
        #[error("An error that may occur upon operating with Linker instances")]
        #[code(104)]
        LinkerError(_message: String),
        #[error("Failed to instantiate and start the Wasm bytecode")]
        #[code(105)]
        InstantiateFailed(_message: String),
        #[error("Global heap base not found")]
        #[code(106)]
        HeapBaseNotFound,
        #[error("Could not find function")]
        #[code(107)]
        FuncNotFound,
        #[error("Invalid number of arguments")]
        #[code(108)]
        InvalidNumArgs,
        #[error("Failed to parse function argument")]
        #[code(109)]
        FailedParseFuncArgs(_message: String),
        #[error("Failed to parse (DataEntry arguments or Payments)")]
        #[code(110)]
        FailedDeserialize,
        #[error("Failed during execution")]
        #[code(111)]
        FailedExec(_message: String),
        #[error("Call stack overflow error")]
        #[code(112)]
        StackOverflow,
        #[error("Failed receiving Module")]
        #[code(113)]
        ModuleNotFound,
        #[error("Fuel metering is disabled")]
        #[code(114)]
        FuelMeteringDisabled(_message: String),
    }
}

// Enumeration of errors that can occur while accessing the JVM.
error! {
    enum JvmError {
        #[error("Failed receiving JVM")]
        #[code(200)]
        JvmNotFound,
        #[error("Failed receiving JVM Callback")]
        #[code(201)]
        JvmCallbackNotFound,
        #[error("Failed attaches the current thread to the Java VM")]
        #[code(202)]
        AttachCurrentThread,
        #[error("Failed JVM method call")]
        #[code(203)]
        MethodCall(_message: String),
        #[error("Failed byte array conversion")]
        #[code(204)]
        ByteArrayConversion,
        #[error("Failed receiving JavaVM interface")]
        #[code(205)]
        GetJavaVM,
        #[error("Error callback new_global_ref")]
        #[code(206)]
        NewGlobalRef,
        #[error("Couldn't create java byte array")]
        #[code(207)]
        NewByteArray,
        #[error("Couldn't create java string")]
        #[code(208)]
        NewString,
        #[error("Failed to receive object")]
        #[code(209)]
        ReceiveObject,
        #[error("Failed to receive byte")]
        #[code(210)]
        ReceiveByte,
        #[error("Failed to receive integer")]
        #[code(211)]
        ReceiveInt,
        #[error("Failed to receive long")]
        #[code(212)]
        ReceiveLong,
        #[error("Failed to receive boolean")]
        #[code(213)]
        ReceiveBoolean,
    }
}

// Enumeration of errors that may occur while working inside the interpreter.
error! {
    enum RuntimeError {
        #[error("Exception thrown during contract execution")]
        #[code(300)]
        Exception(_message: String),
        #[error("Failed receiving Memory")]
        #[code(301)]
        MemoryNotFound,
        #[error("Failed to retrieve UTF-8 string")]
        #[code(302)]
        Utf8Error,
        #[error("Incorrect result obtained during contract execution")]
        #[code(303)]
        InvalidResult(_message: String),
        #[error("Error converting Base58 string to bytes")]
        #[code(304)]
        Base58Error,
        #[error("Error when converting numeric types")]
        #[code(305)]
        ConvertingNumericTypes,
        #[error("AssetHolder type not found")]
        #[code(306)]
        AssetHolderTypeNotFound,
        #[error("Address version not found")]
        #[code(307)]
        AddressVersionNotFound,
        #[error("Value parsing error")]
        #[code(308)]
        ParseError,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let message: String = String::from("Error message");

        assert_eq!(
            ExecutableError::InvalidBytecode(message.clone()).as_jint(),
            100
        );
        assert_eq!(JvmError::JvmNotFound.as_jint(), 200);
        assert_eq!(RuntimeError::Exception(message.clone()).as_jint(), 300);

        assert_eq!(
            ExecutableError::InvalidBytecode(message.clone()).as_i32(),
            100
        );
        assert_eq!(JvmError::JvmNotFound.as_i32(), 200);
        assert_eq!(RuntimeError::Exception(message.clone()).as_i32(), 300);
    }
}
