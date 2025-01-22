use bevy::prelude::*;
mod camera;
mod cursor;
mod osu_asset;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(cursor::CursorPlugin)
        .init_asset_loader::<osu_asset::OsuAssetLoader>()
        .init_asset::<osu_asset::OsuFile>()
        .run();
}
