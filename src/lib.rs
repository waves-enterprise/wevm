mod data_entry;
mod exec;
mod jvm;
mod runtime;
mod stack;

#[cfg(test)]
mod tests;

use crate::{
    exec::{Executable, ExecutableError},
    jvm::JvmError,
    stack::Stack,
};
use jni::{
    objects::{JByteArray, JClass, JObject, JObjectArray, JString},
    sys::jint,
    JNIEnv,
};
use wasmi::core::Value;

#[derive(Debug)]
pub enum Error {
    Jvm(JvmError),
    Executable(ExecutableError),
}

impl Error {
    pub fn as_jint(&self) -> jint {
        match self {
            Error::Jvm(error) => *error as jint,
            Error::Executable(error) => *error as jint,
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

// This `#[no_mangle]` keeps rust from "mangling" the name and making it unique
// for this crate. The name follow a strict naming convention so that the
// JNI implementation will be able to automatically find the implementation
// of a native method based on its name.
//
// The `'local` lifetime here represents the local frame within which any local
// (temporary) references to Java objects will remain valid.
//
// It's usually not necessary to explicitly name the `'local` input lifetimes but
// in this case we want to return a reference and show the compiler what
// local frame lifetime it is associated with.
#[no_mangle]
pub extern "system" fn Java_VM_runContract<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    bytecode: JByteArray<'local>,
    func_name: JString<'local>,
    func_args: JByteArray<'local>,
    callback: JObject<'local>,
) -> jint {
    let bytecode = match env.convert_byte_array(bytecode) {
        Ok(bytecode) => bytecode,
        Err(_) => return JvmError::ByteArrayConversion as jint,
    };

    // TODO: It may be necessary to manage the memory
    let memory: (u32, u32) = (1, 1);
    let envs = runtime::get_envs();

    let jvm = match env.get_java_vm() {
        Ok(jvm) => jvm,
        Err(_) => return JvmError::GetJavaVM as jint,
    };

    let callback = match env.new_global_ref(callback) {
        Ok(callback) => callback,
        Err(_) => return JvmError::NewGlobalRef as jint,
    };

    let mut stack = match Stack::new(bytecode, memory, envs, jvm, callback) {
        Ok(stack) => stack,
        Err(error) => return error.as_jint(),
    };

    let func_name: String = match env.get_string(&func_name) {
        Ok(name) => name.into(),
        Err(_) => return JvmError::NewString as jint,
    };

    let input_data = match env.convert_byte_array(func_args) {
        Ok(input_data) => input_data,
        Err(_) => return JvmError::ByteArrayConversion as jint,
    };

    let result = match stack.run(&func_name, input_data) {
        Ok(result) => result,
        Err(error) => return error.as_jint(),
    };

    match result[0] {
        Value::I32(value) => value as jint,
        _ => 0 as jint,
    }
}

#[no_mangle]
pub extern "system" fn Java_VM_validateBytecode<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    bytecode: JByteArray<'local>,
) -> jint {
    let bytecode = match env.convert_byte_array(bytecode) {
        Ok(bytecode) => bytecode,
        Err(_) => return JvmError::ByteArrayConversion as jint,
    };

    // TODO: It may be necessary to manage the memory
    let memory: (u32, u32) = (1, 1);

    match Executable::new(bytecode, memory.0, memory.1) {
        Ok(_) => 0,
        Err(error) => error.as_jint(),
    }
}
