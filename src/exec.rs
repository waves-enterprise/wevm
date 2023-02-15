use std::fmt;
use std::str::FromStr;

use wasmi::{
    core::{Value, ValueType, F32, F64},
    Config, Engine, Func, FuncType, Memory, MemoryType, Module, Store,
};

use crate::{
    runtime::{Environment, Runtime},
    Error, Result,
};

pub enum LoadableFunction {
    Constructor,
    Call(String),
}

impl FromStr for LoadableFunction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "_constructor" => Ok(Self::Constructor),
            _ => Ok(Self::Call(s.to_string())),
        }
    }
}

impl fmt::Display for LoadableFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadableFunction::Constructor => write!(f, "_constructor"),
            LoadableFunction::Call(name) => write!(f, "{}", name),
        }
    }
}

pub struct Executable<T> {
    module: Module,
    runtime: T,
    /// Initial memory size of a contract's sandbox.
    initial: u32,
    /// The maximum memory size of a contract's sandbox.
    maximum: u32,
}

impl<T: Runtime + Copy> Executable<T> {
    pub fn new(bytecode: Vec<u8>, initial: u32, maximum: u32) -> Result<Self> {
        let module = Self::load_wasm_module(&bytecode)?;
        let runtime = T::new();

        Ok(Executable {
            module,
            runtime,
            initial,
            maximum,
        })
    }

    pub fn call(
        &self,
        func_name: &LoadableFunction,
        func_args: &[String],
        envs: Vec<&impl Environment<T>>,
    ) -> Result<Vec<Value>> {
        let (func, mut store) = Self::load_wasm_func(
            &self.module,
            self.runtime,
            &func_name.to_string(),
            (self.initial, self.maximum),
            envs,
        )?;
        let func_type = func.ty(&store);
        let func_args = Self::type_check_arguments(&func_type, func_args)?;

        let mut results = Self::prepare_results_buffer(&func_type);

        func.call(&mut store, &func_args, &mut results)
            .map_err(|_| Error::FailedExec)?;

        Ok(results)
    }

    fn load_wasm_module(bytecode: &[u8]) -> Result<Module> {
        let mut config = Config::default();
        config
            .wasm_multi_value(false)
            .wasm_mutable_global(false)
            .wasm_sign_extension(false)
            .wasm_saturating_float_to_int(false);

        let engine = Engine::new(&config);
        let module =
            Module::new(&engine, &mut &bytecode[..]).map_err(|_| Error::InvalidBytecode)?;

        Ok(module)
    }

    /// Loads the Wasm [`Func`] from the given Wasm bytecode.
    ///
    /// Returns the [`Func`] together with its [`Store`] for further processing.
    ///
    /// # Errors
    ///
    /// - If the function name argument `func_name` is missing.
    /// - If the Wasm module fails to instantiate or start.
    /// - If the Wasm module does not have an exported function `func_name`.
    fn load_wasm_func(
        module: &Module,
        runtime: T,
        func_name: &str,
        memory: (u32, u32),
        envs: Vec<&impl Environment<T>>,
    ) -> Result<(Func, Store<T>)> {
        let engine = module.engine();
        let mut linker = <wasmi::Linker<()>>::new();
        let mut store = wasmi::Store::new(engine, runtime);

        for env in envs {
            linker
                .define(&env.module(), &env.name(), env.func(&mut store))
                .map_err(|_| Error::LinkerError)?;
        }

        let memory = Memory::new(
            &mut store,
            MemoryType::new(memory.0, Some(memory.1)).map_err(|_| Error::MemoryError)?,
        )
        .map_err(|_| Error::MemoryLimits)?;

        linker
            .define("env", "memory", memory)
            .map_err(|_| Error::LinkerError)?;

        let instance = linker
            .instantiate(&mut store, module)
            .and_then(|pre| pre.start(&mut store))
            .map_err(|_| Error::InstantiateFailed)?;

        store.data_mut().set_memory(memory);

        let func = instance
            .get_export(&store, func_name)
            .and_then(|ext| ext.into_func())
            .ok_or(Error::FuncNotFound)?;

        Ok((func, store))
    }

    /// Type checks the given function arguments and returns them decoded into [`Value`]s.
    ///
    /// # Errors
    ///
    /// - If the number of given arguments is not equal to the number of function parameters.
    /// - If an argument cannot be properly parsed to its expected parameter type.
    fn type_check_arguments(func_type: &FuncType, func_args: &[String]) -> Result<Vec<Value>> {
        if func_type.params().len() != func_args.len() {
            return Err(Error::InvalidNumArgs);
        }

        let func_args = func_type
            .params()
            .iter()
            .zip(func_args)
            .enumerate()
            .map(|(_, (param_type, arg))| {
                macro_rules! make_err {
                    () => {
                        |_| Error::FailedParseFuncArgs
                    };
                }

                match param_type {
                    ValueType::I32 => arg.parse::<i32>().map(Value::from).map_err(make_err!()),
                    ValueType::I64 => arg.parse::<i64>().map(Value::from).map_err(make_err!()),
                    ValueType::F32 => arg
                        .parse::<f32>()
                        .map(F32::from)
                        .map(Value::from)
                        .map_err(make_err!()),
                    ValueType::F64 => arg
                        .parse::<f64>()
                        .map(F64::from)
                        .map(Value::from)
                        .map_err(make_err!()),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(func_args)
    }

    /// Returns a [`Value`] buffer capable of holding the return values.
    fn prepare_results_buffer(func_type: &FuncType) -> Vec<Value> {
        func_type
            .results()
            .iter()
            .copied()
            .map(Value::default)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use convert_case::{Case, Casing};
    use std::str;
    use wasmi::Caller;

    use crate::env_runtime;
    use crate::tests::{wat2wasm, MockRuntime};

    const MEMORY: (u32, u32) = (1, 1);

    env_runtime! {
        pub fn Test<MockRuntime>() {
            |mut _caller: Caller<MockRuntime>| {
                assert_eq!(2 + 2, 4);
            }
        }
    }

    #[test]
    fn test_simple_module() {
        let wat = r#"
            (module
                (func $getValue (result i32)
                    i32.const 42)
                (func (export "run") (result i32)
                    call $getValue
                    i32.const 1
                    i32.add))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let exec = Executable::new(bytecode, MEMORY.0, MEMORY.1).expect("Error load wasm bytecode");

        let func_name = LoadableFunction::from_str("run").expect("Error parse func name");
        let func_args: [String; 0] = [];

        let env = Test;

        let result = exec
            .call(&func_name, &func_args, vec![&env])
            .expect("Error execution");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Value::I32(43));
    }

    env_runtime! {
        pub fn TestSetValue<MockRuntime>(value: u32) {
            |mut _caller: Caller<MockRuntime>| {
                assert_eq!(13, value);
            }
        }
    }

    #[test]
    fn test_import_set_value_module() {
        let wat = r#"
            (module
                (import "env" "test_set_value" (func $setValue (param i32)))
                (func (export "run")
                    i32.const 13
                    call $setValue))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let exec = Executable::new(bytecode, MEMORY.0, MEMORY.1).expect("Error load wasm bytecode");

        let func_name = LoadableFunction::from_str("run").expect("Error parse func name");
        let func_args: [String; 0] = [];

        let env = TestSetValue;

        let result = exec
            .call(&func_name, &func_args, vec![&env])
            .expect("Error execution");

        assert_eq!(result.len(), 0);
    }

    env_runtime! {
        pub fn TestGetValue<MockRuntime>() -> u32 {
            |mut _caller: Caller<MockRuntime>| {
                13
            }
        }
    }

    #[test]
    fn test_import_get_value_module() {
        let wat = r#"
            (module
                (import "env" "test_get_value" (func $getValue (result i32)))
                (func (export "run") (result i32)
                    call $getValue
                    i32.const 1
                    i32.add))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let exec = Executable::new(bytecode, MEMORY.0, MEMORY.1).expect("Error load wasm bytecode");

        let func_name = LoadableFunction::from_str("run").expect("Error parse func name");
        let func_args: [String; 0] = [];

        let env = TestGetValue;

        let result = exec
            .call(&func_name, &func_args, vec![&env])
            .expect("Error execution");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Value::I32(14));
    }

    env_runtime! {
        pub fn TestMemory<MockRuntime>(offset: u32, length: u32) {
            |mut caller: Caller<MockRuntime>| {
                let (memory, _ctx) = caller
                    .data()
                    .memory()
                    .expect("Error get memory")
                    .data_and_store_mut(&mut caller);

                let result = str::from_utf8(&memory[offset as usize..offset as usize + length as usize])
                    .expect("Error converts a slice of bytes to a string slice");

                assert_eq!("Hi", result);
            }
        }
    }

    #[test]
    fn test_memory_module() {
        let wat = r#"
            (module
                (import "env" "test_memory" (func $print (param i32 i32)))
                (import "env" "memory" (memory 1 1))
                (data (i32.const 0) "Hi")
                (func (export "run")
                    i32.const 0  ;; pass offset 0 to print
                    i32.const 2  ;; pass length 2 to print
                    call $print))
        "#;

        let bytecode = wat2wasm(wat).expect("Error parse wat");
        let exec = Executable::new(bytecode, MEMORY.0, MEMORY.1).expect("Error load wasm bytecode");

        let func_name = LoadableFunction::from_str("run").expect("Error parse func name");
        let func_args: [String; 0] = [];

        let env = TestMemory;

        let result = exec
            .call(&func_name, &func_args, vec![&env])
            .expect("Error execution");

        assert_eq!(result.len(), 0);
    }
}
