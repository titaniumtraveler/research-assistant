use research_assistant::config::{
    strip_root, Config, ResolvedConfig, ResolvedSourceFileConfig, ResolvedSourceFormat,
};
use serde::{ser::SerializeMap, Serialize};
use std::fmt::Debug;

fn main() -> anyhow::Result<()> {
    match run() {
        Ok(str) => println!("{str}"),
        Err(err) => {
            println!("Err {err}");
            println!("Err {err:#?}");
        }
    }

    Ok(())
}

#[derive(Serialize)]
struct ListSources {
    config: ResolvedConfig,
    sources: Vec<Sources>,
}

enum StrRes<T> {
    V(T),
    Err(String),
}

impl<T, E: Debug> From<Result<T, E>> for StrRes<T> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(t) => Self::V(t),
            Err(err) => Self::Err(format!("{err:?}")),
        }
    }
}

impl<T: Serialize> Serialize for StrRes<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StrRes::V(v) => Serialize::serialize(v, serializer),
            StrRes::Err(err) => serializer.serialize_str(err),
        }
    }
}

#[derive(Serialize)]
struct Sources {
    source: ResolvedSourceFileConfig,
    files: Vec<StrRes<Source>>,
}

struct Source {
    source: String,
    format: ResolvedSourceFormat,
}

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.source, &self.format)?;
        map.end()
    }
}

fn run() -> anyhow::Result<String> {
    let config = Config::read_config()?.resolve()?;
    let str = serde_json::to_string_pretty(&ListSources {
        sources: config
            .source
            .clone()
            .into_iter()
            .map(move |source| Sources {
                files: source
                    .source_files()
                    .map(|files| {
                        files
                            .map(|(file, config)| Source {
                                source: strip_root(&source.root, &file).display().to_string(),
                                format: config,
                            })
                            .into()
                    })
                    .collect::<Vec<StrRes<_>>>(),
                source,
            })
            .collect(),
        config,
    })?;

    Ok(str)
}
