
mod graphics;
mod object;

use anyhow::{bail, Result};
use object::Object;
use std::env;
use std::fs;
use std::str::FromStr;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1])?;
    let _object = match Object::from_str(&file) {
        Ok(object) => object,
        Err(_) => bail!("Failed to parse the obj file"),
    };

    Ok(())
}
