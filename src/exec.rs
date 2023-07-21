use crate::{
    data_entry,
    runtime::{Environment, Runtime, RuntimeError},
    stack::Stack,
    Error, Result,
};
use std::{fmt, str::FromStr};
use wasmi::{
    core::{Value, ValueType, F32, F64},
    Config, Engine, Func, FuncType, Memory, MemoryType, Module, StackLimits, Store,
};

#[derive(Copy, Clone, Debug)]
pub enum ExecutableError {
    /// Failed to parse and validate Wasm bytecode
    InvalidBytecode = 100,
    /// Could not found constructor
    ConstructorNotFound = 101,
    /// An error that may occur upon operating with virtual or linear memory
    MemoryError = 102,
    /// Limits limit the amount of memory well below u32::MAX
    MemoryLimits = 103,
    /// An error that may occur upon operating with Linker instances
    LinkerError = 104,
    /// Failed to instantiate and start the Wasm bytecode
    InstantiateFailed = 105,
    /// Could not find function
    FuncNotFound = 106,
    /// invalid number of arguments
    InvalidNumArgs = 107,
    /// Failed to parse function argument
    FailedParseFuncArgs = 108,
    /// Failed to parse DataEntry arguments
    FailedParseDataEntry = 109,
    /// Failed during execution
    FailedExec = 110,
}

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

pub struct Executable {
    module: Module,
    /// Initial memory size of a contract's sandbox.
    initial: u32,
    /// The maximum memory size of a contract's sandbox.
    maximum: u32,
}

impl Executable {
    pub fn new(bytecode: Vec<u8>, initial: u32, maximum: u32) -> Result<Self> {
        let stack_limits = StackLimits::default();

        let mut config = Config::default();
        config
            .set_stack_limits(stack_limits)
            .wasm_multi_value(false)
            .wasm_mutable_global(false)
            .wasm_sign_extension(false)
            .wasm_saturating_float_to_int(false);

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

        Ok(Executable {
            module,
            initial,
            maximum,
        })
    }

    pub fn execute(
        &self,
        func_name: &LoadableFunction,
        input_data: Vec<u8>,
        envs: Vec<Box<dyn Environment>>,
        stack: &mut Stack,
    ) -> Result<Vec<Value>> {
        let runtime = Runtime::new(stack);

        let (func, mut store) = Self::load_wasm_func(
            &self.module,
            runtime,
            &func_name.to_string(),
            (self.initial, self.maximum),
            envs,
        )?;

        let memory = match store.data().memory() {
            Some(memory) => memory,
            None => return Err(Error::Runtime(RuntimeError::MemoryNotFound)),
        };

        let mut offset_memory: usize = match memory.current_pages(&mut store).to_bytes() {
            Some(offset) => offset,
            None => return Err(Error::Executable(ExecutableError::MemoryError)),
        };

        let array_memory = memory.data_mut(&mut store);

        let func_args: Vec<String> =
            data_entry::parse(input_data.as_slice(), array_memory, &mut offset_memory)?;

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
        envs: Vec<Box<dyn Environment>>,
    ) -> Result<(Func, Store<Runtime<'a>>)> {
        let engine = module.engine();
        let mut linker = <wasmi::Linker<()>>::new();
        let mut store = wasmi::Store::new(engine, runtime);

        for env in envs {
            linker
                .define(&env.module(), &env.name(), env.func(&mut store))
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
    use crate::tests::wat2wasm;
    use jni::sys::jint;

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
        let memory: (u32, u32) = (1, 1);

        let exec = Executable::new(bytecode, memory.0, memory.1);
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
        let memory: (u32, u32) = (1, 1);

        let exec = Executable::new(bytecode, memory.0, memory.1);
        assert!(exec.is_err());
    }

    #[test]
    fn test_error() {
        assert_eq!(ExecutableError::InvalidBytecode as jint, 100);
    }
}
