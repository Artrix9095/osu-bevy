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
            (tick_game_timer, handle_music_start, spawn_hitobjects),
        )
        .init_resource::<GameState>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct GameState {
    timer: Stopwatch,
    music_started: bool,
    current_object_index: usize,
}

#[derive(Component)]
pub struct HitCircle {
    spawn_time: f32,
    position: Vec2,
}

fn tick_game_timer(time: Res<Time>, mut game_state: ResMut<GameState>) {
    game_state.timer.tick(time.delta());
}

fn handle_music_start(
    mut game_state: ResMut<GameState>,
    mut audio_query: Query<&mut AudioSink, With<MasterAudio>>,
) {
    dbg!("{:#?}", &game_state);
    if !game_state.music_started {
        dbg!("handle_music_start");
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

        while game_state.current_object_index < hit_objects.len() {
            let hit_object = &hit_objects[game_state.current_object_index];
            let current_time = time.elapsed_secs();

            // Calculate spawn time based on approach rate
            let ar = beatmap.0.approach_rate;
            let preempt = ar_to_ms(ar);

            if current_time >= hit_object.start_time as f32 - preempt {
                match hit_object.kind {
                    HitObjectKind::Circle(hit_object) => {
                        let position = Vec2::new(hit_object.pos.x, hit_object.pos.y);
                        let circle = meshes.add(Circle::new(10.0));
                        commands.spawn((
                            HitCircle {
                                spawn_time: current_time,
                                position,
                            },
                            Mesh2d(circle),
                            MeshMaterial2d(materials.add(Color::rgb(0.0, 0.0, 1.0))),
                            Transform::from_xyz(position.x, position.y, 3.0),
                            // Add any other components needed for rendering
                        ));

                        game_state.current_object_index += 1;
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }
    }
}
