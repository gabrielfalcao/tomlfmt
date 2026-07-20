use crate::Result;
use iocore::Path;
use toml::Value;

pub fn prettify_file(path: &Path, write_in_place: bool) -> Result<()> {
    let source = path.read()?;
    let loaded = toml::from_str::<Value>(&source)?;
    let prettified = toml::to_string_pretty(&loaded)?;
    if source != prettified {
        if write_in_place {
            path.write(prettified.as_bytes())?;
            eprintln!("wrote prettified {path}");
        } else {
            println!("{prettified}")
        }
    } else {
        eprintln!("nothing to do for {path}");
    }
    Ok(())
}
