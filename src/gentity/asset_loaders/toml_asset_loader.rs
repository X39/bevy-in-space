use bevy::app::App;
use bevy::asset::{Asset, AssetApp, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::Reader;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use thiserror::Error;
use toml::Table;
use crate::gentity::asset_loaders::rhai_asset_loader::{RhaiScript};
use crate::localization::Localization;


#[derive(Default)]
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset_loader::<TomlAssetLoader>()
        ;
    }
}

#[derive(Default, Debug)]
pub struct TomlAssetLocalizationEntry {
    key: String,
    value: String,
}

#[derive(Default, Debug)]
pub struct TomlAssetLocalization {
    culture: String,
    entries: Vec<TomlAssetLocalizationEntry>,
}

#[derive(Default, Debug)]
pub struct TomlAssetDisplay {
    title: String,
    description: String,
}

#[derive(Default, Asset, TypePath, Debug)]
pub struct TomlAsset {
    pub identifier: String,
    pub gltf: String,
    pub display: TomlAssetDisplay,
    pub localizations: Vec<TomlAssetLocalization>,
    pub gltf_asset: Handle<Scene>,
    pub script_assets: Vec<Handle<RhaiScript>>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TomlAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Could not create encoding using encoding_rs")]
    CreatingEncodingFailed,
    #[error("Invalid characters found in script")]
    InvalidCharactersFound(String),
    #[error("Could not read path from asset")]
    ReadingPathFailed,
    #[error("The file is empty")]
    EmptyFile,
    #[error("Failed reading identifier as not a string")]
    FailedReadingIdentifierAsNotAString,
    #[error("Failed reading identifier as not present")]
    FailedReadingIdentifierNotFound,
    #[error("Failed reading gltf as not a string")]
    FailedReadingGltfAsNotAString,
    #[error("Failed reading gltf as not present")]
    FailedReadingGltfNotFound,
    #[error("One or more localizations failed to be read as culture is not a string")]
    OneOrMoreLocalizationsFailedToBeReadAsCultureIsNotAString,
    #[error("One or more localizations failed to be read as culture not found")]
    OneOrMoreLocalizationsFailedToBeReadAsCultureNotFound,
}


#[derive(Default)]
pub struct TomlAssetLoader;

impl AssetLoader for TomlAssetLoader {
    type Asset = TomlAsset;
    type Settings = ();
    type Error = TomlAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // Read the path of the asset
            let path_str_opt = load_context.path().to_str();
            let Some(path_str) = path_str_opt else {
                return Err(TomlAssetLoaderError::ReadingPathFailed);
            };
            let path = path_str.to_string();
            let base_path = path[..path.rfind("/").unwrap_or(0)].to_string();

            // Load script contents
            let mut bytes = Vec::new();
            let result = reader.read_to_end(&mut bytes).await?;
            if result == 0 {
                return Err(TomlAssetLoaderError::EmptyFile);
            }
            let encoding_opt = encoding_rs::Encoding::for_bom(bytes.as_slice());
            let Some((encoding, _)) = encoding_opt else {
                return Err(TomlAssetLoaderError::CreatingEncodingFailed);
            };
            let (text, _, success) = encoding.decode(bytes.as_slice());
            let script = text.to_string();
            if !success {
                return Err(TomlAssetLoaderError::InvalidCharactersFound(script));
            }

            let table = match script.parse::<toml::Table>() {
                Ok(table) => table,
                Err(err) => return Err(TomlAssetLoaderError::Toml(err)),
            };
            let identifier = match Self::read_identifier_from_toml(&table) {
                Ok(value) => value,
                Err(value) => return Err(value),
            };

            let gltf = match Self::read_gltf_from_toml(&table) {
                Ok(value) => value,
                Err(value) => return Err(value),
            };

            let gltf_asset = Self::load_gltf_assets(load_context, &base_path, &gltf);

            // Read all script assets in base_path/scripts
            let script_assets = match Self::load_script_assets(load_context, base_path) {
                Ok(value) => value,
                Err(value) => return Err(value),
            };

            let display = Self::read_display_from_toml(&table);

            let localizations = match Self::read_localization_from_toml(table) {
                Ok(value) => value,
                Err(value) => return Err(value),
            };

            // Create the asset
            let custom_asset = TomlAsset {
                identifier,
                gltf,
                display,
                localizations,
                gltf_asset,
                script_assets,
            };

            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}

impl TomlAssetLoader {
    fn read_localization_from_toml(table: Table) -> Result<Vec<TomlAssetLocalization>, TomlAssetLoaderError> {
        let localizations = match table.get("localizations") {
            Some(localizations) => match localizations.as_array() {
                Some(localizations) => {
                    let mut localizations_vec = Vec::new();
                    for localization in localizations {
                        let culture = match localization.get("culture") {
                            Some(culture) => match culture.as_str() {
                                Some(culture) => culture.to_string(),
                                None => return Err(TomlAssetLoaderError::OneOrMoreLocalizationsFailedToBeReadAsCultureIsNotAString),
                            },
                            None => return Err(TomlAssetLoaderError::OneOrMoreLocalizationsFailedToBeReadAsCultureNotFound),
                        };
                        let entries = match localization.get("entries") {
                            Some(entries) => match entries.as_array() {
                                Some(entries) => {
                                    let mut entries_vec = Vec::new();
                                    for entry in entries {
                                        let key = match entry.get("key") {
                                            Some(key) => match key.as_str() {
                                                Some(key) => key.to_string(),
                                                None => continue,
                                            },
                                            None => continue,
                                        };
                                        let value = match entry.get("value") {
                                            Some(value) => match value.as_str() {
                                                Some(value) => value.to_string(),
                                                None => continue,
                                            },
                                            None => continue,
                                        };
                                        entries_vec.push(TomlAssetLocalizationEntry {
                                            key,
                                            value,
                                        });
                                    }
                                    entries_vec
                                }
                                None => Vec::new(),
                            },
                            None => Vec::new(),
                        };
                        localizations_vec.push(TomlAssetLocalization {
                            culture,
                            entries,
                        });
                    }
                    localizations_vec
                }
                None => vec![],
            },
            None => vec![],
        };
        Ok(localizations)
    }

    fn read_display_from_toml(table: &Table) -> TomlAssetDisplay {
        let display = match table.get("display") {
            Some(display) => match display.as_table() {
                Some(display) => {
                    let title = match display.get("title") {
                        Some(title) => match title.as_str() {
                            Some(title) => title.to_string(),
                            None => "".into(),
                        },
                        None => "".into(),
                    };
                    let description = match display.get("description") {
                        Some(description) => match description.as_str() {
                            Some(description) => description.to_string(),
                            None => "".into(),
                        },
                        None => "".into(),
                    };
                    TomlAssetDisplay {
                        title,
                        description,
                    }
                }
                None => TomlAssetDisplay { ..default() },
            },
            None => TomlAssetDisplay { ..default() },
        };
        display
    }

    fn load_script_assets(load_context: &mut LoadContext, base_path: String) -> Result<Vec<Handle<RhaiScript>>, TomlAssetLoaderError> {
        let scripts_path = format!("{}/scripts", base_path);
        let scripts_path = std::path::Path::new(&scripts_path);
        let mut script_assets = Vec::new();
        if scripts_path.exists() {
            for entry in std::fs::read_dir(scripts_path)? {
                let entry = entry?;
                let path = entry.path();
                let path_str = path.to_str();
                let Some(path_str) = path_str else {
                    continue;
                };
                let path_str = path_str.to_string();
                let script_asset: Handle<RhaiScript> = load_context.load(path_str);
                script_assets.push(script_asset);
            }
        }
        Ok(script_assets)
    }

    fn load_gltf_assets(load_context: &mut LoadContext, base_path: &String, gltf: &String) -> Handle<Scene> {
        let gltf_path = format!("{}/{}.gltf", base_path, gltf);
        let gltf_asset: Handle<Scene> = load_context.load(gltf_path);
        gltf_asset
    }

    fn read_gltf_from_toml(table: &Table) -> Result<String, TomlAssetLoaderError> {
        let gltf = match table.get("gltf") {
            Some(gltf) => match gltf.as_str() {
                Some(gltf) => gltf.to_string(),
                None => return Err(TomlAssetLoaderError::FailedReadingGltfAsNotAString),
            },
            None => return Err(TomlAssetLoaderError::FailedReadingGltfNotFound),
        };
        Ok(gltf)
    }

    fn read_identifier_from_toml(table: &Table) -> Result<String, TomlAssetLoaderError> {
        let identifier = match table.get("identifier") {
            Some(identifier) => match identifier.as_str() {
                Some(identifier) => identifier.to_string(),
                None => return Err(TomlAssetLoaderError::FailedReadingIdentifierAsNotAString),
            },
            None => return Err(TomlAssetLoaderError::FailedReadingIdentifierNotFound),
        };
        Ok(identifier)
    }
}

pub fn toml_asset_changed(
    mut event_reader: EventReader<AssetEvent<TomlAsset>>,
    mut assets: ResMut<Assets<TomlAsset>>,
    mut localization: ResMut<Localization>,
) {
    for ev in event_reader.read() {
        match ev {
            AssetEvent::Added { .. } => {}
            AssetEvent::Modified { .. } => {}
            AssetEvent::Removed { .. } => {}
            AssetEvent::LoadedWithDependencies { id } => {
                let opt = assets.get_mut(*id);
                let Some(toml_asset) = opt else {
                    panic!("TOML asset should be loaded at this point but it isn't");
                };
                for asset_localization in toml_asset.localizations {
                    for entry in asset_localization.entries {
                        localization.set(&asset_localization.culture, entry.key, entry.value);
                    }
                }
            }
        }
    }
}