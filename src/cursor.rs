use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use bevy::winit::WinitSettings;

use crate::camera::MainCamera;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_systems(Startup, setup_cursor)
            .add_systems(Update, cursor_follow);
    }
}

// Move the circle to the cursor position
fn cursor_follow(
    mut cursor: Query<&mut Transform, With<Cursor>>,
    mut windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let mut cursor = cursor.single_mut();
    let (camera, camera_transform) = camera.single();
    let cursor_transform = windows.single_mut().cursor_position();
    if let Some(dt) = cursor_transform {
        let ray = camera.viewport_to_world(camera_transform, dt).unwrap();
        let ray = ray.origin.truncate();
        cursor.translation.x = ray.x;
        cursor.translation.y = ray.y;
    }
}

#[derive(Component)]
struct Cursor;

fn setup_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    hide_cursor(windows);

    // Spawn a white circle.
    let circle = meshes.add(Circle {
        radius: 10.0,
        ..Default::default()
    });
    commands
        .spawn((
            Mesh2d(circle),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
            Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ))
        .insert(Cursor);
}

pub fn hide_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = &mut primary_window.single_mut();
    window.cursor_options.visible = false;
}
