pub struct GameplayPlugin;
use crate::beatmap::{BeatmapResource, MasterAudio};
use crate::osu_asset::OsuFile;
use crate::util::ar_to_ms;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rosu_map::section::hit_objects::*;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_game_timer,
                handle_music_start,
                spawn_hitobjects,
                update_hitobjects,
            ),
        )
        .init_resource::<GameState>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct GameState {
    timer: Stopwatch,
    music_started: bool,
    spawned_objects: bool,
}

#[derive(Component)]
pub struct HitObject(
    /// Approach rate window
    pub f32,
    /// Hit window
    pub f32,
);
#[derive(Component)]
pub struct SliderBody;

fn tick_game_timer(time: Res<Time>, mut game_state: ResMut<GameState>) {
    game_state.timer.tick(time.delta());
}

fn handle_music_start(
    mut game_state: ResMut<GameState>,
    mut audio_query: Query<&mut AudioSink, With<MasterAudio>>,
) {
    if !game_state.music_started {
        if let Ok(mut playback) = audio_query.get_single_mut() {
            playback.play();

            game_state.music_started = true;
        }
    }
}

fn spawn_hitobjects(
    mut commands: Commands,
    time: Res<Time>,
    beatmap_assets: Res<Assets<OsuFile>>,
    beatmap_resource: Res<BeatmapResource>,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !game_state.music_started {
        return;
    }

    if let Some(beatmap) = beatmap_assets.get(&beatmap_resource.beatmap) {
        let hit_objects = &beatmap.0.hit_objects;
        let ar = &beatmap.0.approach_rate;
        let od = &beatmap.0.overall_difficulty;

        for (_, hit_object) in hit_objects.iter().enumerate() {
            match hit_object.kind {
                HitObjectKind::Circle(circle) => {
                    commands.spawn((
                        HitObject(hit_object.start_time as f32, 0.0),
                        Mesh2d(meshes.add(Circle::new(10.0))),
                        MeshMaterial2d(materials.add(Color::srgba(0.0, 0.0, 1.0, 0.0))),
                        Transform::from_xyz(circle.pos.x, circle.pos.y, 3.0),
                    ));
                }
                HitObjectKind::Slider(slider) => {
                    commands.spawn((
                        HitObject(hit_object.start_time as f32, 0.0),
                        Mesh2d(meshes.add(Circle::new(10.0))),
                        MeshMaterial2d(materials.add(Color::srgba(0.0, 0.0, 1.0, 0.0))),
                        Transform::from_xyz(slider.pos.x, slider.pos.y, 3.0),
                        SliderBody,
                    ));
                }
                _ => {}
            }
        }
        game_state.spawned_objects = true;
    }
}

pub fn update_hitobjects(
    mut query: Query<(&mut HitObject, &mut MeshMaterial2d<ColorMaterial>)>,
    time: Res<Time>,
    beatmap_assets: Res<Assets<OsuFile>>,
    beatmap_resource: Res<BeatmapResource>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut game_state: ResMut<GameState>,
) {
    if !game_state.spawned_objects {
        return;
    }
    if let Some(beatmap) = beatmap_assets.get(&beatmap_resource.beatmap) {
        let ar = ar_to_ms(beatmap.0.approach_rate);
        for (mut hit_object, mut mesh) in query.iter_mut() {
            let object_start_time = (ar / 2.0) - hit_object.0;
            let object_end_time = (ar / 2.0) + hit_object.0;

            if time.elapsed_secs() - object_start_time >= 0.0 {
                mesh.0 = materials.add(Color::srgba(0.0, 0.0, 1.0, 1.0));
            } else if time.elapsed_secs() > object_end_time {
                mesh.0 = materials.add(Color::srgba(0.0, 0.0, 1.0, 0.0));
            }
        }
    }
}
