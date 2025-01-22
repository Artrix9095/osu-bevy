use crate::osu_asset::OsuFile;
use bevy::{asset::LoadedFolder, prelude::*};

#[derive(Resource, Default)]
pub struct BeatmapResource {
    pub beatmap: Handle<OsuFile>,
    pub background: Handle<Image>,
    pub audio: Handle<AudioSource>,
}

pub struct BeatmapPlugin;

impl Plugin for BeatmapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BeatmapResource>()
            .add_systems(Startup, load_beatmap)
            .add_systems(Update, handle_beatmap_loading);
    }
}

#[derive(Resource, Default)]
struct BeatmapLoadState {
    loaded: bool,
}

fn load_beatmap(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the beatmap file
    let beatmap_handle = asset_server.load("beatmap/beatmap.osu");

    // Create the resource with unloaded handles
    commands.insert_resource(BeatmapResource {
        beatmap: beatmap_handle,
        background: Handle::default(),
        audio: Handle::default(),
    });

    commands.insert_resource(BeatmapLoadState::default());
}

fn handle_beatmap_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    beatmap_assets: Res<Assets<OsuFile>>,
    mut beatmap_resource: ResMut<BeatmapResource>,
    mut load_state: ResMut<BeatmapLoadState>,
    audio_assets: Res<Assets<AudioSource>>,
) {
    // Only process once when beatmap is loaded
    if load_state.loaded {
        return;
    }

    // Check if beatmap is loaded
    if let Some(beatmap) = beatmap_assets.get(&beatmap_resource.beatmap) {
        // Load background
        let bg_file = &beatmap.0.background_file;
        let bg_path = format!("beatmap/{}", bg_file);
        beatmap_resource.background = asset_server.load(&bg_path);

        // Spawn background sprite
        commands.spawn((
            Sprite::from_image(beatmap_resource.background.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        // Load audio
        let audio_file = &beatmap.0.audio_file;
        let audio_path = format!("beatmap/{}", audio_file);
        beatmap_resource.audio = asset_server.load(&audio_path);

        // Spawn audio source
        commands.spawn((
            AudioPlayer::new(beatmap_resource.audio.clone()),
            PlaybackSettings::ONCE,
        ));

        load_state.loaded = true;
    }
}
