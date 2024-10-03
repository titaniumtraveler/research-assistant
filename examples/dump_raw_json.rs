use anyhow::{Context, Result};
use research_assistant::{config::Config, reader::Reader};

pub fn main() -> Result<()> {
    match run() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Err {err}");
            eprintln!("Err {err:#?}");
        }
    }

    Ok(())
}

fn run() -> Result<()> {
    let config = Config::read_config()?.resolve()?;
    let mut reader = Reader::new();
    let mut sources = Vec::with_capacity(config.source.len());
    for source in config.source {
        for file in source.source_files() {
            let (entry, format) = file?;
            let v: serde_json::Value = reader
                .deserialize_from(entry.path(), format)
                .with_context(|| entry.path().display().to_string())?;
            sources.push(v);
        }
    }
    println!("{}", serde_json::to_string(&sources)?);
    Ok(())
}
