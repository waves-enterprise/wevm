use crate::stack::Stack;
use jni::objects::{JByteArray, JObject, JValue};

pub trait JVM {
    fn jvm_get_bytecode(&self, name: &str) -> Vec<u8>;
}

// Implementing the JVM call
// A special note that this implementation did not overlap with the test implementation
#[cfg(not(test))]
impl JVM for Stack {
    fn jvm_get_bytecode(&self, name: &str) -> Vec<u8> {
        let mut env = self
            .jvm
            .attach_current_thread()
            .expect("Failed attaches the current thread to the Java VM");

        let name = env.new_string(name).expect("Couldn't create java string");

        let result = env
            .call_method(
                self.jvm_callback.clone(),
                "getBytecode",
                "(Ljava/lang/String;)[B",
                &[JValue::Object(&name.into())],
            )
            .expect("Failed JVM method call")
            .l()
            .expect("Failed to receive object");

        let bytes = env
            .convert_byte_array(<JObject<'_> as Into<JByteArray>>::into(result))
            .expect("Failed byte array conversion");

        bytes.to_vec()
    }
}
