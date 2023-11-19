use bevy::asset::{AssetLoader, AsyncReadExt};
use bevy::prelude::*;
use serde::Deserialize;
use thiserror::*;

use super::Portrait;

#[derive(Debug)]
pub struct DialogLine {
    pub portrait: Portrait,
    pub text: Box<str>,
    pub speaker: Box<str>,
}

#[derive(Debug, Default, TypePath, Asset)]
pub struct DialogAsset {
    pub lines: Vec<DialogLine>,
}

#[derive(Debug, Deserialize)]
struct DialogFile {
    lines: Vec<(Portrait, String, String)>,
}

#[derive(Debug, Error)]
pub enum DialogLoadError {
    #[error("Could not load dialog: {0}")]
    IO(#[from] std::io::Error),
    #[error("Could not parse ron: {0}")]
    Parsing(#[from] ron::error::SpannedError),
}

#[derive(Debug, Default)]
pub struct DialogLoader;

impl AssetLoader for DialogLoader {
    type Asset = DialogAsset;

    type Settings = ();

    type Error = DialogLoadError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut buf = vec![];
            reader.read_to_end(&mut buf).await?;
            let dialog_file: DialogFile = ron::de::from_bytes(&buf)?;
            let lines = dialog_file
                .lines
                .into_iter()
                .map(|(portrait, speaker, text)| DialogLine {
                    portrait,
                    speaker: speaker.into(),
                    text: text.into(),
                })
                .collect();

            let dialog = DialogAsset { lines };

            Ok(dialog)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dialog.ron"]
    }
}
