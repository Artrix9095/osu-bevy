use bevy::{
    prelude::*,
    remote::{http::RemoteHttpPlugin, RemotePlugin},
};
mod beatmap;
mod camera;
mod cursor;
mod gameplay;
mod osu_asset;
mod util;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()))
        .init_asset_loader::<osu_asset::OsuAssetLoader>()
        .init_asset::<osu_asset::OsuFile>()
        .add_plugins(camera::CameraPlugin)
        .add_plugins(cursor::CursorPlugin)
        .add_plugins(beatmap::BeatmapPlugin)
        .add_plugins(gameplay::GameplayPlugin)
        .run();
}
