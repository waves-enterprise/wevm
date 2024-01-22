# wevm
Waves Enterprise Virtual Machine for WASM smart-contracts

WEVM represents:
* WebAssembly interpreter (Wasm)
* Functions for working with Node
* Interface for interaction with Node
* A mechanism for controlling the execution of smart contracts

WEVM uses:
* Crate [wasmi](https://docs.rs/wasmi/0.23.0/wasmi/index.html) is used as the WebAssembly interpreter
* Java Native Interface (JNI) and crate [jni](https://docs.rs/jni/0.21.0/jni/struct.JavaVM.html) are used as a way for WEVM to communicate with Waves Enterprise Node

## Development

At the root is the implementation of a Scala interface for integration into Waves Enterprise Node. In the `native` folder there is an implementation of WEVM in Rust language.

### Run WEVM test
```
cargo test --features jvm
```

### Run Scala test
```
sbt buildWAT
sbt test
```
