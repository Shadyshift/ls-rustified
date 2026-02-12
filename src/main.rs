use std::fs;
use std::path::Path;
use std::error::Error;

fn run(dir: &Path) -> Result<(), Box<dyn Error>> {
	for entry in fs::read_dir(dir)?{
      let entry = entry?;
      println!("{}", entry.file_name().to_string_lossy());
	}
	Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    run(Path::new("."))?;
    Ok(())
}

