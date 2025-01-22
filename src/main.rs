use bevy::prelude::*;
mod beatmap;
mod camera;
mod cursor;
mod osu_asset;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_asset_loader::<osu_asset::OsuAssetLoader>()
        .init_asset::<osu_asset::OsuFile>()
        .add_plugins(camera::CameraPlugin)
        .add_plugins(cursor::CursorPlugin)
        .add_plugins(beatmap::BeatmapPlugin)
        .run();
}
