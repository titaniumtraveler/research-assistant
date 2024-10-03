use anyhow::{Context, Result};
use encoding_rs::Encoding;
use globset::{Glob, GlobMatcher};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

mod impl_serde;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    source: Option<Vec<SourceFileConfig>>,
}

impl Config {
    pub fn read_config() -> Result<Self> {
        Self::read_config_from_path("config.toml")
    }

    pub fn read_config_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let config = read_to_string(path).with_context(|| path.display().to_string())?;
        toml::from_str(&config).map_err(Into::into)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceFileConfig {
    pub root: PathBuf,
    pub follow_links: Option<bool>,
    pub max_depth: Option<usize>,
    pub glob: Option<SourceGlob>,
    pub format: Option<SourceFormat>,
    pub files: Option<BTreeMap<PathBuf, SourceFormat>>,
    pub exclude: Option<BTreeSet<PathBuf>>,
}

#[derive(Debug)]
pub enum SourceGlob {
    Glob(String),
    Enable(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFormat {
    /// Whether to enable bom sniffing, defaults to `true`
    pub autodetect: Option<bool>,
    /// Which character encoding to use based on <https://encoding.spec.whatwg.org/#concept-encoding-get>
    pub encoding: Option<&'static Encoding>,
    pub allow_trailing_comma: Option<bool>,
}

impl SourceFormat {
    fn resolve(format: Option<SourceFormat>) -> ResolvedSourceFormat {
        let default = ResolvedSourceFormat {
            autodetect: true,
            encoding: None,
            allow_trailing_comma: true,
        };
        match format {
            None => default,
            Some(SourceFormat {
                autodetect,
                encoding,
                allow_trailing_comma,
            }) => ResolvedSourceFormat {
                autodetect: autodetect.unwrap_or(default.autodetect),
                encoding: encoding.or(default.encoding),
                allow_trailing_comma: allow_trailing_comma.unwrap_or(default.allow_trailing_comma),
            },
        }
    }
}

impl Config {
    pub fn resolve(self) -> Result<ResolvedConfig> {
        match self {
            Config {
                source: Some(sources),
            } => Ok(ResolvedConfig {
                source: sources
                    .into_iter()
                    .map(
                        |SourceFileConfig {
                             root,
                             follow_links,
                             max_depth,
                             glob,
                             format,
                             files,
                             exclude,
                         }| {
                            Ok(ResolvedSourceFileConfig {
                                root: shellexpand::path::full(&root)?
                                    .canonicalize()
                                    .with_context(|| root.display().to_string())?,
                                follow_links: follow_links.unwrap_or(false),
                                max_depth: max_depth.unwrap_or(usize::MAX),
                                glob: match glob {
                                    Some(SourceGlob::Enable(false)) => ResolvedSourceGlob::Disabled,
                                    Some(SourceGlob::Glob(glob)) => ResolvedSourceGlob::Glob(
                                        Glob::new(&glob)?.compile_matcher(),
                                    ),
                                    Some(SourceGlob::Enable(true)) | None => {
                                        ResolvedSourceGlob::Glob(
                                            Glob::new("*/*.json")?.compile_matcher(),
                                        )
                                    }
                                },
                                format: SourceFormat::resolve(format),
                                files: match files {
                                    None => BTreeMap::new(),
                                    Some(map) => map
                                        .into_iter()
                                        .map(|(k, v)| Ok((k, SourceFormat::resolve(Some(v)))))
                                        .collect::<Result<BTreeMap<PathBuf, ResolvedSourceFormat>>>(
                                        )?,
                                },
                                exclude: exclude.unwrap_or_default(),
                            })
                        },
                    )
                    .collect::<Result<Vec<_>>>()?,
            }),
            Config { source: None } => Ok(ResolvedConfig { source: Vec::new() }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedConfig {
    pub source: Vec<ResolvedSourceFileConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedSourceFileConfig {
    pub root: PathBuf,
    pub follow_links: bool,
    pub max_depth: usize,
    pub glob: ResolvedSourceGlob,
    pub format: ResolvedSourceFormat,
    pub files: BTreeMap<PathBuf, ResolvedSourceFormat>,
    pub exclude: BTreeSet<PathBuf>,
}

pub fn strip_root<'a>(root: &Path, entry: &'a DirEntry) -> &'a Path {
    entry
        .path()
        .strip_prefix(root)
        .expect("expected root to be always be a prefix of path")
}

impl ResolvedSourceFileConfig {
    pub fn source_files(
        &self,
    ) -> impl Iterator<Item = walkdir::Result<(DirEntry, ResolvedSourceFormat)>> + '_ {
        let root = &self.root;
        let glob = match &self.glob {
            ResolvedSourceGlob::Glob(glob) => Some(glob),
            ResolvedSourceGlob::Disabled => None,
        };
        WalkDir::new(&self.root)
            .max_depth(self.max_depth)
            .into_iter()
            .filter_entry(|entry| !self.exclude.contains(strip_root(root, entry)))
            .filter_map(move |entry| match entry {
                Ok(entry) => match entry.file_type().is_file() {
                    false => None,
                    true => {
                        let path = strip_root(root, &entry);
                        if !glob
                            .as_ref()
                            .map(|glob| glob.is_match(path))
                            .unwrap_or(true)
                        {
                            return None;
                        }

                        let config = self.files.get(path).unwrap_or(&self.format);

                        Some(Ok((entry, *config)))
                    }
                },
                Err(err) => Some(Err(err)),
            })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResolvedSourceFormat {
    pub autodetect: bool,
    pub encoding: Option<&'static Encoding>,
    pub allow_trailing_comma: bool,
}

#[derive(Debug, Clone)]
pub enum ResolvedSourceGlob {
    Glob(GlobMatcher),
    Disabled,
}
