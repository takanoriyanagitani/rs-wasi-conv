use std::io::{stdin, BufRead, BufReader};

use wasmedge_sdk::config::{
    CommonConfigOptions, Config, ConfigBuilder, HostRegistrationConfigOptions,
};
use wasmedge_sdk::{Module, Vm};

use wasmedge_types::ValType;

fn run_func(vm: &Vm, m: &str, f: &str) -> Result<(), String> {
    let results = vm
        .run_func(Some(m), f, vec![])
        .map_err(|e| format!("Unable to run func({}/{}): {}", m, f, e))?;
    let res = results
        .get(0)
        .ok_or_else(|| format!("Unable to get result({}/{})", m, f))?;
    let code: i32 = res
        .ty()
        .eq(&ValType::I32)
        .then(|| res.to_i32())
        .ok_or_else(|| format!("Unexpected result type({}/{}): {:?}", m, f, res.ty()))?;
    code.eq(&0)
        .then(|| ())
        .ok_or_else(|| format!("Conversion non0 exit code: {}", code))
}

fn get_module_names() -> Vec<String> {
    let i = stdin();
    let il = i.lock();
    let ib = BufReader::new(il);
    let lines = ib.lines();
    lines.flat_map(|r| r.ok()).collect()
}

fn sub() -> Result<(), String> {
    let cfg: Config = ConfigBuilder::new(CommonConfigOptions::new())
        .with_host_registration_config(HostRegistrationConfigOptions::new().wasi(true))
        .build()
        .map_err(|e| format!("Unable to build config: {}", e))?;
    let vm = Vm::new(Some(cfg)).map_err(|e| format!("Unable to get vm: {}", e))?;
    let names = get_module_names();

    let mut vm = names.iter().try_fold(vm, |v, name| {
        let m = Module::from_file(None, name)
            .map_err(|e| format!("Unable to get module({}): {}", name, e))?;
        v.register_module(Some(name), m)
            .map_err(|e| format!("Unable to register module({}): {}", name, e))
    })?;
    let mut wm = vm
        .wasi_module()
        .map_err(|e| format!("Unable to get wasi module: {}", e))?;
    wm.initialize(
        None,
        Some(vec![
            "ENV_INPUT_FILENAME=/guest/data/in.dat",
            "ENV_OUTPUT_FILENAME=/guest/data/out.dat",
        ]),
        Some(vec!["/guest/data:./"]),
    );
    for name in &names {
        run_func(&vm, name, "convert")?;
        std::fs::rename("./out.dat", "./in.dat")
            .map_err(|e| format!("Unable to rename: {}", e))?;
    }
    std::fs::rename("./in.dat", "./out.dat").map_err(|e| format!("Unable to rename: {}", e))?;
    Ok(())
}

fn main() {
    match sub() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
