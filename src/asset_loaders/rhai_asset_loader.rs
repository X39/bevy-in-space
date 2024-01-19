use bevy::app::App;
use bevy::asset::{Asset, AssetApp, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::Reader;
use bevy::reflect::TypePath;
use thiserror::Error;
use serde::{Deserialize, Serialize};
// https://github.com/TheLeonsver1/bevy_rhai/blob/master/examples/minimal.rs


#[derive(Default)]
pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<RhaiAssetLoader>();
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
    #[error("Could not create encoding using encoding_rs")]
    CreatingEncodingFailed,
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
            let encoding_opt = encoding_rs::Encoding::for_bom(bytes.as_slice());
            let Some((encoding, _)) = encoding_opt else {
                return Err(RhaiAssetLoaderError::CreatingEncodingFailed);
            };
            let (text, _, success) = encoding.decode(bytes.as_slice());
            let script = text.to_string();
            if !success {
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
