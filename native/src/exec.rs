use crate::{
    error::{Error, ExecutableError, Result, RuntimeError},
    modules::Module as M,
    runtime::{data_entry::DataEntry, Runtime},
    vm::Vm,
};
use std::{fmt, str::FromStr};
use wasmi::{
    core::ValueType, Config, Engine, Func, FuncType, Memory, MemoryType, Module, StackLimits,
    Store, Value,
};

/// Enumeration of possible executable functions of a WASM contract.
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

#[derive(Debug)]
pub struct Executable {
    module: Option<Module>,
    /// Initial memory size of a contract's sandbox.
    initial: u32,
    /// The maximum memory size of a contract's sandbox.
    maximum: u32,
    /// Fuel limit for contract execution.
    fuel_limit: u64,
}

impl Executable {
    /// Initializing the WASM contract executable.
    pub fn new(initial: u32, maximum: u32, fuel_limit: u64) -> Self {
        Self {
            module: None,
            initial,
            maximum,
            fuel_limit,
        }
    }

    /// Load bytecode for execution.
    pub fn load_bytecode(&mut self, bytecode: &[u8]) -> Result<()> {
        self.module = Some(Self::create_module(bytecode)?);
        Ok(())
    }

    /// Validate bytecode contract.
    pub fn validate_bytecode(bytecode: &[u8]) -> Result<Module> {
        Self::create_module(bytecode)
    }

    /// Initializing `Engine` and `Module`.
    fn create_module(bytecode: &[u8]) -> Result<Module> {
        let stack_limits = StackLimits::default();

        let mut config = Config::default();
        config
            .set_stack_limits(stack_limits)
            .wasm_mutable_global(false)
            .wasm_sign_extension(false)
            .wasm_saturating_float_to_int(false)
            .wasm_multi_value(true)
            .floats(false)
            .consume_fuel(true);

        let engine = Engine::new(&config);
        let module = Module::new(&engine, &mut &bytecode[..])
            .map_err(|_| Error::Executable(ExecutableError::InvalidBytecode))?;

        if module
            .exports()
            .filter(|item| item.name() == LoadableFunction::Constructor.to_string())
            .count()
            != 1
        {
            return Err(Error::Executable(ExecutableError::ConstructorNotFound));
        }

        Ok(module)
    }

    /// Execution of the WASM contract function.
    pub fn execute(
        &self,
        func_name: &LoadableFunction,
        params: &[u8],
        modules: Vec<M>,
        vm: &mut Vm,
    ) -> Result<Vec<Value>> {
        let runtime = Runtime::new(vm);

        let (func, mut store) = Self::load_wasm_func(
            self.module
                .as_ref()
                .ok_or(Error::Executable(ExecutableError::ModuleNotFound))?,
            runtime,
            &func_name.to_string(),
            (self.initial, self.maximum),
            self.fuel_limit,
            modules,
        )?;

        let memory = match store.data().memory() {
            Some(memory) => memory,
            None => return Err(Error::Runtime(RuntimeError::MemoryNotFound)),
        };
        let mut offset_memory = store.data().heap_base() as usize;
        let array_memory = memory.data_mut(&mut store);

        let func_args: Vec<String> =
            DataEntry::deserialize_params(params, array_memory, &mut offset_memory)?;

        store.data_mut().set_heap_base(offset_memory as i32);

        let func_type = func.ty(&store);
        let func_args = Self::type_check_arguments(&func_type, func_args.as_slice())?;

        let mut results = Self::prepare_results_buffer(&func_type);

        func.call(&mut store, &func_args, &mut results)
            .map_err(|_| Error::Executable(ExecutableError::FailedExec))?;

        Ok(results)
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
    fn load_wasm_func<'a>(
        module: &Module,
        runtime: Runtime<'a>,
        func_name: &str,
        memory: (u32, u32),
        fuel_limit: u64,
        modules: Vec<M>,
    ) -> Result<(Func, Store<Runtime<'a>>)> {
        let engine = module.engine();
        let mut linker = <wasmi::Linker<Runtime>>::new(engine);
        let mut store = wasmi::Store::new(engine, runtime);

        for item in modules {
            let (module, name, func) = item(&mut store);
            linker
                .define(&module, &name, func)
                .map_err(|_| Error::Executable(ExecutableError::LinkerError))?;
        }

        let memory = Memory::new(
            &mut store,
            MemoryType::new(memory.0, Some(memory.1))
                .map_err(|_| Error::Executable(ExecutableError::MemoryError))?,
        )
        .map_err(|_| Error::Executable(ExecutableError::MemoryLimits))?;

        linker
            .define("env", "memory", memory)
            .map_err(|_| Error::Executable(ExecutableError::LinkerError))?;

        let instance = linker
            .instantiate(&mut store, module)
            .and_then(|pre| pre.start(&mut store))
            .map_err(|_| Error::Executable(ExecutableError::InstantiateFailed))?;

        store.data_mut().set_memory(memory);
        store
            .add_fuel(fuel_limit)
            .map_err(|_| Error::Executable(ExecutableError::FuelMeteringDisabled))?;

        let heap_base = match instance.get_global(&mut store, "__heap_base") {
            Some(global) => match global.get(&mut store) {
                Value::I32(value) => value,
                _ => return Err(Error::Executable(ExecutableError::HeapBaseNotFound)),
            },
            None => return Err(Error::Executable(ExecutableError::HeapBaseNotFound)),
        };

        store.data_mut().set_heap_base(heap_base);

        let func = instance
            .get_export(&store, func_name)
            .and_then(|ext| ext.into_func())
            .ok_or(Error::Executable(ExecutableError::FuncNotFound))?;

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
            return Err(Error::Executable(ExecutableError::InvalidNumArgs));
        }

        let func_args = func_type
            .params()
            .iter()
            .zip(func_args)
            .enumerate()
            .map(|(_, (param_type, arg))| {
                macro_rules! make_err {
                    () => {
                        |_| Error::Executable(ExecutableError::FailedParseFuncArgs)
                    };
                }

                match param_type {
                    ValueType::I32 => arg.parse::<i32>().map(Value::from).map_err(make_err!()),
                    ValueType::I64 => arg.parse::<i64>().map(Value::from).map_err(make_err!()),
                    _ => Err(Error::Executable(ExecutableError::FailedParseFuncArgs)),
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
    use crate::tests::wat2wasm;

    #[test]
    fn test_executable_valid_bytecode() {
        let wat = r#"
        (module
            (type $t0 (func (result i32)))
            (func $_constructor (export "_constructor") (type $t0) (result i32)
                (i32.add
                    (i32.const 2)
                    (i32.const 2)
                )
            )
        )
        "#;

        let bytecode = wat2wasm(wat).expect("WAT code parsing failed");
        let exec = Executable::validate_bytecode(&bytecode);

        assert!(exec.is_ok());
    }

    #[test]
    fn test_executable_invalid_bytecode() {
        let wat = r#"
        (module
            (type $t0 (func (result i32)))
            (func $run (export "run") (type $t0) (result i32)
                (i32.add
                    (i32.const 2)
                    (i32.const 2)
                )
            )
        )
        "#;

        let bytecode = wat2wasm(wat).expect("WAT code parsing failed");
        let exec = Executable::validate_bytecode(&bytecode);

        assert!(exec.is_err());
        assert_eq!(
            exec.unwrap_err(),
            Error::Executable(ExecutableError::ConstructorNotFound)
        );
    }
}
