use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

use crate::event::PartialEvent;

mod event;

#[no_mangle]
fn convert() -> i32 {
    env_convert().map(|_| 0).unwrap_or(1)
}

fn try_convert<P>(i: P, o: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let mut r = File::open(&i).map_err(|e| format!("Unable to open input csv: {}", e))?;
    let mut w = File::create(&o).map_err(|e| format!("Unable to create output csv: {}", e))?;
    let bw = BufWriter::new(&w);
    reader2writer(&mut r, bw, new_key_filter("temp".into()))?;
    w.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}

fn env_convert() -> Result<(), String> {
    let i: String = env::var("ENV_INPUT_FILENAME")
        .map_err(|e| format!("Unable to get input filename: {}", e))?;
    let o: String = env::var("ENV_OUTPUT_FILENAME")
        .map_err(|e| format!("Unable to get output filename: {}", e))?;
    try_convert(i, o)
}

fn new_key_filter(key: String) -> impl Fn(&PartialEvent) -> bool {
    move |evt| evt.key_eq(key.as_str())
}

fn reader2writer<R, W, F>(r: &mut R, mut w: W, f: F) -> Result<(), String>
where
    R: Read,
    W: Write,
    F: Fn(&PartialEvent) -> bool,
{
    let mut cr = csv::Reader::from_reader(r);
    let parsed = cr.deserialize::<PartialEvent>();
    let noerr = parsed.flat_map(|r| r.ok());
    let mut filtered = noerr.filter(f);
    filtered.try_for_each(|evt: PartialEvent| {
        serde_json::to_writer(&mut w, &evt).map_err(|e| format!("Unable to write event: {}", e))?;
        writeln!(&mut w).map_err(|e| format!("Unable to write: {}", e))
    })?;
    w.flush().map_err(|e| format!("Unable to flush: {}", e))?;
    Ok(())
}
