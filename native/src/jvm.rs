use crate::{stack::Stack, Error, Result};
use jni::objects::{JByteArray, JObject, JValue};

#[derive(Copy, Clone, Debug)]
pub enum JvmError {
    /// Failed attaches the current thread to the Java VM
    AttachCurrentThread = 200,
    /// Couldn't create java byte array
    NewByteArray = 201,
    /// Failed JVM method call
    MethodCall = 202,
    /// Failed to receive object
    ReceiveObject = 203,
    /// Failed byte array conversion
    ByteArrayConversion = 204,
    /// Failed receiving JavaVM interface
    GetJavaVM = 205,
    /// Error callback new_global_ref
    NewGlobalRef = 206,
    /// Couldn't create java string
    NewString = 207,
}

pub trait Jvm {
    fn get_bytecode(&self, contract_id: &[u8]) -> Result<Vec<u8>>;
}

// Implementing the JVM call
// A special note that this implementation did not overlap with the test implementation
#[cfg(not(test))]
impl Jvm for Stack {
    fn get_bytecode(&self, contract_id: &[u8]) -> Result<Vec<u8>> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .map_err(|_| Error::Jvm(JvmError::AttachCurrentThread))?;

        let contract_id = env
            .byte_array_from_slice(contract_id)
            .map_err(|_| Error::Jvm(JvmError::NewByteArray))?;

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getBytecode",
                "([B)[B",
                &[JValue::Object(&contract_id.into())],
            )
            .map_err(|_| Error::Jvm(JvmError::MethodCall))?
            .l()
            .map_err(|_| Error::Jvm(JvmError::ReceiveObject))?;

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .map_err(|_| Error::Jvm(JvmError::ByteArrayConversion))?;

        Ok(bytes.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jni::sys::jint;

    #[test]
    fn test_error() {
        assert_eq!(JvmError::AttachCurrentThread as jint, 200);
    }
}
