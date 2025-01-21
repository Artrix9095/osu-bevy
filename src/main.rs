use bevy::prelude::*;
mod camera;
mod cursor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(cursor::CursorPlugin)
        .run();
}
