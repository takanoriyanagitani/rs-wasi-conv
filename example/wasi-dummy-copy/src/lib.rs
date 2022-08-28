use std::env;
use std::path::Path;

#[no_mangle]
pub fn convert() -> i32 {
    env_convert().map(|_| 0).unwrap_or(1)
}

fn try_convert<P>(i: P, o: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    std::fs::copy(i, o)
        .map(|_| ())
        .map_err(|e| format!("Unable to copy: {}", e))
}

fn env_convert() -> Result<(), String> {
    let i: String = env::var("ENV_INPUT_FILENAME")
        .map_err(|e| format!("Unable to get input filename: {}", e))?;
    let o: String = env::var("ENV_OUTPUT_FILENAME")
        .map_err(|e| format!("Unable to get output filename: {}", e))?;
    try_convert(i, o)
}
