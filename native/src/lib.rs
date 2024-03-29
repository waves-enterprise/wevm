mod env;
mod error;
mod exec;
mod modules;
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
    sys::{jint, jlong},
    JNIEnv,
};
use wasmi::Value;

/// Size of allocated linear memory.
pub const MEMORY: (u32, u32) = (2, 16);

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
    params: JByteArray<'local>,
    fuel_limit: jlong,
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

    let modules = modules::all();

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
        fuel_limit as u64,
        modules,
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

    let params = match env.convert_byte_array(params) {
        Ok(bytes) => bytes,
        Err(_) => return JvmError::ByteArrayConversion as jint,
    };

    let result = match vm.run(&func_name, &params) {
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

    match Executable::validate_bytecode(&bytecode) {
        Ok(_) => 0,
        Err(error) => error.as_jint(),
    }
}
