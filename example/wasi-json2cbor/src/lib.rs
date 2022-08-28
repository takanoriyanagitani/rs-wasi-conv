use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

use serde_json::{Map, Value};

#[no_mangle]
fn convert() -> i32 {
    env_convert().map(|_| 0).unwrap_or(1)
}

fn str2map(s: &str) -> Option<Map<String, Value>> {
    serde_json::from_str(s).ok()
}

fn try_convert<P>(i: P, o: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let mut i = File::open(i).map_err(|e| format!("Unable to open input json: {}", e))?;
    let mut o = File::create(o).map_err(|e| format!("Unable to create output cbor: {}", e))?;
    let bw = BufWriter::new(&o);
    reader2writer(&mut i, bw)?;
    o.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}

fn env_convert() -> Result<(), String> {
    let i: String = env::var("ENV_INPUT_FILENAME")
        .map_err(|e| format!("Unable to get input filename: {}", e))?;
    let o: String = env::var("ENV_OUTPUT_FILENAME")
        .map_err(|e| format!("Unable to get output filename: {}", e))?;
    try_convert(i, o)
}

fn reader2writer<R, W>(r: &mut R, mut w: W) -> Result<(), String>
where
    R: Read,
    W: Write,
{
    let br = BufReader::new(r);
    let lines = br.lines().flat_map(|r| r.ok());
    let mut parsed = lines.flat_map(|line| str2map(line.as_str()));
    parsed.try_for_each(|m| {
        ciborium::ser::into_writer(&m, &mut w)
            .map_err(|e| format!("Unable to write as cbor: {}", e))
    })?;
    w.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}
