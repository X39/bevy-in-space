use bevy::app::App;
use bevy::asset::{Asset, AssetApp, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::Reader;
use bevy::reflect::TypePath;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use crate::gentity::asset_loaders::toml_asset_loader::TomlAssetLoaderError;
// https://github.com/TheLeonsver1/bevy_rhai/blob/master/examples/minimal.rs


#[derive(Default)]
pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {

        app
            .init_asset::<RhaiScript>()
            .init_asset_loader::<RhaiAssetLoader>()
        ;
    }
}

#[derive(Asset, TypePath, Debug, Deserialize, Serialize)]
pub struct RhaiScript {
    pub content: String,
    pub path: String,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RhaiAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid characters found in script")]
    InvalidCharactersFound(String),
    #[error("Could not read path from asset")]
    ReadingPathFailed,
}


#[derive(Default)]
pub struct RhaiAssetLoader;

impl AssetLoader for RhaiAssetLoader {
    type Asset = RhaiScript;
    type Settings = ();
    type Error = RhaiAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // Read the path of the asset
            let path_str_opt = load_context.path().to_str();
            let Some(path_str) = path_str_opt else {
                return Err(RhaiAssetLoaderError::ReadingPathFailed);
            };
            let path = path_str.to_string();

            // Load script contents
            let mut bytes = Vec::new();
            let result = reader.read_to_end(&mut bytes).await?;
            if result == 0 {
                return Ok(RhaiScript {
                    content: String::new(),
                    path,
                });
            }
            let encoding = encoding_rs::UTF_8;
            let (text, _, has_malformed_characters) = encoding.decode(bytes.as_slice());
            let script = text.to_string();
            if has_malformed_characters {
                return Err(RhaiAssetLoaderError::InvalidCharactersFound(script));
            }

            // Create the asset
            let custom_asset = RhaiScript {
                content: script,
                path,
            };
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["rhai"]
    }
}
