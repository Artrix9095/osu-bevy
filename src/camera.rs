use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}
// Setup a 2d camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}
