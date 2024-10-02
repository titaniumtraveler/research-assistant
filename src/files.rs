use crate::config::{ResolvedSourceFileConfig, ResolvedSourceFormat};
use walkdir::{DirEntry, WalkDir};

pub struct SourceFiles<'a, I> {
    config: &'a ResolvedSourceFileConfig,
    iter: I,
}

impl<I> SourceFiles<'_, I> {
    pub fn new(
        config: &ResolvedSourceFileConfig,
    ) -> SourceFiles<'_, impl Iterator<Item = walkdir::Result<(DirEntry, ResolvedSourceFormat)>> + '_>
    {
        let iter = WalkDir::new(&config.root)
            .max_depth(config.max_depth)
            .into_iter()
            .filter_entry(|entry| !config.exclude.contains(entry.path()))
            .filter_map(|entry| match entry {
                Ok(entry) => match entry.file_type().is_file() {
                    false => None,
                    true => {
                        let config = config.files.get(entry.path()).unwrap_or(&config.format);

                        Some(Ok((entry, *config)))
                    }
                },
                Err(err) => Some(Err(err)),
            });

        SourceFiles { config, iter }
    }
}
