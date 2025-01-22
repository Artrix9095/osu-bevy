use anyhow::*;
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use rosu_map::{from_bytes, Beatmap};

#[derive(Asset, TypePath, Debug)]
pub struct OsuFile(pub Beatmap);

#[derive(Default)]
pub struct OsuAssetLoader;

impl AssetLoader for OsuAssetLoader {
    type Asset = OsuFile;
    type Settings = ();
    type Error = anyhow::Error;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let map = from_bytes(&bytes)?;

        let asset = OsuFile(map);

        Ok(asset)
    }
    fn extensions(&self) -> &[&str] {
        &["osu"]
    }
}
