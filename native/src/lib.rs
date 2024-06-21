#[cfg(feature = "jvm")]
mod env;

#[cfg(feature = "jvm")]
mod error;

#[cfg(feature = "jvm")]
mod exec;

#[cfg(feature = "jvm")]
mod jvm;

mod modules;

#[cfg(feature = "jvm")]
mod node;

#[cfg(feature = "jvm")]
mod runtime;

#[cfg(all(test, feature = "jvm"))]
mod tests;

#[cfg(feature = "jvm")]
mod vm;

pub use modules::v0;
pub use modules::v1;

#[cfg(feature = "jvm")]
use crate::{error::JvmError, exec::Executable, vm::Vm};
#[cfg(feature = "jvm")]
use base58::ToBase58;
#[cfg(feature = "jvm")]
use jni::{
    objects::{JByteArray, JClass, JObject, JString},
    sys::{jint, jlong},
    JNIEnv,
};
#[cfg(feature = "jvm")]
use log::{debug, error};
#[cfg(feature = "jvm")]
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
#[cfg(feature = "jvm")]
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
    let _ = env_logger::try_init();
    let contract_id = match env.convert_byte_array(contract_id) {
        Ok(bytes) => bytes,
        Err(_) => {
            error!("{}", JvmError::ByteArrayConversion);
            return JvmError::ByteArrayConversion.as_jint();
        }
    };

    debug!(
        "Started WEVM to execute the contract: {}",
        contract_id.clone().to_base58()
    );

    let bytecode = match env.convert_byte_array(bytecode) {
        Ok(bytes) => bytes,
        Err(_) => {
            error!("{}", JvmError::ByteArrayConversion);
            return JvmError::ByteArrayConversion.as_jint();
        }
    };

    let jvm = match env.get_java_vm() {
        Ok(jvm) => jvm,
        Err(_) => {
            error!("{}", JvmError::GetJavaVM);
            return JvmError::GetJavaVM.as_jint();
        }
    };

    let callback = match env.new_global_ref(callback) {
        Ok(callback) => callback,
        Err(_) => {
            error!("{}", JvmError::NewGlobalRef);
            return JvmError::NewGlobalRef.as_jint();
        }
    };

    let mut vm = match Vm::new(
        contract_id,
        bytecode,
        MEMORY,
        fuel_limit as u64,
        modules(),
        Some(jvm),
        Some(callback),
    ) {
        Ok(vm) => vm,
        Err(error) => {
            error!("{}", error);
            return error.as_jint();
        }
    };

    let func_name: String = match env.get_string(&func_name) {
        Ok(string) => string.into(),
        Err(_) => {
            error!("{}", JvmError::NewString);
            return JvmError::NewString.as_jint();
        }
    };

    let params = match env.convert_byte_array(params) {
        Ok(bytes) => bytes,
        Err(_) => {
            error!("{}", JvmError::ByteArrayConversion);
            return JvmError::ByteArrayConversion.as_jint();
        }
    };

    let result = match vm.run(&func_name, &params) {
        Ok(result) => result,
        Err(error) => {
            error!("{}", error);
            return error.as_jint();
        }
    };

    match result[0] {
        Value::I32(value) => value as jint,
        _ => 0 as jint,
    }
}

/// External Java function to validate bytecode contract.
#[cfg(feature = "jvm")]
#[no_mangle]
pub extern "system" fn Java_com_wavesenterprise_wasm_core_WASMExecutor_validateBytecode<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    bytecode: JByteArray<'local>,
) -> jint {
    let bytecode = match env.convert_byte_array(bytecode) {
        Ok(bytes) => bytes,
        Err(_) => {
            error!("{}", JvmError::ByteArrayConversion);
            return JvmError::ByteArrayConversion.as_jint();
        }
    };

    match Executable::validate_bytecode(&bytecode) {
        Ok(_) => 0,
        Err(error) => {
            error!("{}", error);
            error.as_jint()
        }
    }
}

#[cfg(feature = "jvm")]
fn modules() -> Vec<modules::Module> {
    let mut vec = vec![];
    vec.extend(v0::modules::modules());
    vec.extend(v1::modules::modules());
    vec
}
