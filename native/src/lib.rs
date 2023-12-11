mod data_entry;
mod env;
mod error;
mod exec;
mod macros;
mod node;
mod runtime;
mod vm;

#[cfg(feature = "jvm")]
mod jvm;

#[cfg(test)]
mod tests;

use crate::{error::JvmError, exec::Executable, vm::Vm};
use jni::{
    objects::{JByteArray, JClass, JObject, JString},
    sys::jint,
    JNIEnv,
};
use wasmi::core::Value;

/// Size of allocated linear memory.
const MEMORY: (u32, u32) = (2, 16);

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

/// External Java function to execute bytecode contract.
#[no_mangle]
pub extern "system" fn Java_com_wavesenterprise_wasm_core_WASMExecutor_runContract<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    contract_id: JByteArray<'local>,
    bytecode: JByteArray<'local>,
    func_name: JString<'local>,
    func_args: JByteArray<'local>,
    callback: JObject<'local>,
) -> jint {
    let contract_id = match env.convert_byte_array(contract_id) {
        Ok(bytes) => bytes,
        Err(_) => return JvmError::ByteArrayConversion as jint,
    };

    let bytecode = match env.convert_byte_array(bytecode) {
        Ok(bytes) => bytes,
        Err(_) => return JvmError::ByteArrayConversion as jint,
    };

    let envs = env::envs();

    let jvm = match env.get_java_vm() {
        Ok(jvm) => jvm,
        Err(_) => return JvmError::GetJavaVM as jint,
    };

    let callback = match env.new_global_ref(callback) {
        Ok(callback) => callback,
        Err(_) => return JvmError::NewGlobalRef as jint,
    };

    let mut vm = match Vm::new(
        contract_id,
        bytecode,
        MEMORY,
        envs,
        Some(jvm),
        Some(callback),
    ) {
        Ok(vm) => vm,
        Err(error) => return error.as_jint(),
    };

    let func_name: String = match env.get_string(&func_name) {
        Ok(string) => string.into(),
        Err(_) => return JvmError::NewString as jint,
    };

    let input_data = match env.convert_byte_array(func_args) {
        Ok(bytes) => bytes,
        Err(_) => return JvmError::ByteArrayConversion as jint,
    };

    let result = match vm.run(&func_name, input_data) {
        Ok(result) => result,
        Err(error) => return error.as_jint(),
    };

    match result[0] {
        Value::I32(value) => value as jint,
        _ => 0 as jint,
    }
}

/// External Java function to validate bytecode contract.
#[no_mangle]
pub extern "system" fn Java_com_wavesenterprise_wasm_core_WASMExecutor_validateBytecode<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    bytecode: JByteArray<'local>,
) -> jint {
    let bytecode = match env.convert_byte_array(bytecode) {
        Ok(bytes) => bytes,
        Err(_) => return JvmError::ByteArrayConversion as jint,
    };

    match Executable::new(bytecode, MEMORY.0, MEMORY.1) {
        Ok(_) => 0,
        Err(error) => error.as_jint(),
    }
}
