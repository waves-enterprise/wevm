mod exec;
mod runtime;
mod stack;

#[cfg(test)]
mod tests;

use crate::stack::Stack;
use jni::{
    objects::{JByteArray, JClass, JObject, JObjectArray, JString},
    sys::jint,
    JNIEnv,
};
use wasmi::core::Value;

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
    _func_args: JObjectArray<'local>,
    callback: JObject<'local>,
) -> jint {
    let bytecode = env
        .convert_byte_array(&bytecode)
        .expect("Failed get byte[] out of java");

    // TODO: It may be necessary to manage the memory
    let memory: (u32, u32) = (1, 1);
    let envs = runtime::get_envs();

    let jvm = env
        .get_java_vm()
        .expect("Failed receiving JavaVM interface");
    let callback = env
        .new_global_ref(callback)
        .expect("Error callback new_global_ref");

    let mut stack =
        Stack::new(bytecode, memory, envs, jvm, callback).expect("Call stack initiation failed");

    let func_name: String = env
        .get_string(&func_name)
        .expect("Couldn't get java string")
        .into();

    // TODO: Parse args
    let result = stack
        .run(&func_name, &[])
        .expect("Bytecode execution failed");

    match result[0] {
        Value::I32(value) => value as jint,
        _ => 0 as jint,
    }
}
