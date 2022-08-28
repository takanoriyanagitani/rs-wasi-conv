###### 1. install wasi runtime

  - WasmEdge: supported
  - Wasmtime: not supported(for now)
  - Wasmer: not supported(for now)

###### 2. build your wasi modules(wasm files)

  - function name: "convert"
  - arguments: void
  - return: i32
  - input filename: get from environment variable(ENV_INPUT_FILENAME)
  - output filename: get from environment variable(ENV_OUTPUT_FILENAME)

###### 3. list wasi modules(*.wasm) and execute rs-wasi-conv

  - rs-wasi-conv reads stdin to get module names to execute
  - rs-wasi-conv calls 'convert' function of 1st module (which reads "in.dat" and writes "out.dat")
  - rs-wasi-conv rename "out.dat" to "in.dat"
  - rs-wasi-conv calls 'convert' function of 2nd module
  - rs-wasi-conv rename "out.dat" to "in.dat"
  - rs-wasi-conv calls 'convert' function of 3rd module
  - rs-wasi-conv rename "out.dat" to "in.dat"
  - ...
  - rs-wasi-conv calls 'convert' function of nth module
  - rs-wasi-conv rename "out.dat" to "in.dat"
  - rs-wasi-conv calls 'convert' function of final module
  - out.dat will be created
