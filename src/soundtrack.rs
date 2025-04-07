use bevy::prelude::*;

use crate::{Settings, app_state::AppState};

const FADE_TIME: f32 = 2.0;

pub struct SoundtrackPlugin;

impl Plugin for SoundtrackPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SoundtrackHandles>()
            .register_type::<FadeIn>()
            .register_type::<FadeOut>()
            .register_type::<AudioPlayer>()
            .add_event::<SoundtrackToggled>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    fade_in,
                    fade_out,
                    handle_soundtrack_toggle.run_if(on_event::<SoundtrackToggled>),
                ),
            )
            .add_systems(OnEnter(AppState::Menu), play_menu_soundtrack)
            .add_systems(OnEnter(AppState::Game), play_game_soundtrack);
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct SoundtrackHandles {
    menu: Handle<AudioSource>,
    game: Handle<AudioSource>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundtrackHandles {
        menu: asset_server.load("soundtracks/Fun_Adventure.ogg"),
        game: asset_server.load("soundtracks/Underground.ogg"),
    });
}

fn spawn_track(commands: &mut Commands, track: Handle<AudioSource>) {
    commands.spawn((
        Name::new("Soundtrack"),
        AudioPlayer(track),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::ZERO,
            ..default()
        },
        FadeIn,
    ));
}

fn play_menu_soundtrack(
    mut commands: Commands,
    tracks: Query<Entity, With<AudioSink>>,
    track_handles: Res<SoundtrackHandles>,
    settings: Res<Settings>,
) {
    if settings.soundtrack_enabled {
        for track in &tracks {
            commands.entity(track).insert(FadeOut).remove::<FadeIn>();
        }
        spawn_track(&mut commands, track_handles.menu.clone());
    }
}

fn play_game_soundtrack(
    mut commands: Commands,
    tracks: Query<Entity, With<AudioSink>>,
    track_handles: Res<SoundtrackHandles>,
    settings: Res<Settings>,
) {
    if settings.soundtrack_enabled {
        for track in &tracks {
            commands.entity(track).insert(FadeOut).remove::<FadeIn>();
        }
        spawn_track(&mut commands, track_handles.game.clone());
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FadeIn;
#[derive(Component, Reflect)]
#[reflect(Component)]
struct FadeOut;

fn fade_in(mut commands: Commands, mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeIn>>, time: Res<Time>) {
    for (audio, entity) in audio_sink.iter_mut() {
        audio.set_volume(audio.volume() + time.delta_secs() / FADE_TIME);
        if audio.volume() >= 1.0 {
            audio.set_volume(1.0);
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn fade_out(mut commands: Commands, mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeOut>>, time: Res<Time>) {
    for (audio, entity) in audio_sink.iter_mut() {
        audio.set_volume(audio.volume() - time.delta_secs() / FADE_TIME);
        if audio.volume() <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Event)]
pub struct SoundtrackToggled;

fn handle_soundtrack_toggle(
    mut commands: Commands,
    tracks: Query<Entity, With<AudioSink>>,
    track_handles: Res<SoundtrackHandles>,
    app_state: Res<State<AppState>>,
    settings: Res<Settings>,
) {
    match settings.soundtrack_enabled {
        true => spawn_track(
            &mut commands,
            match app_state.get() {
                AppState::Menu => track_handles.menu.clone(),
                _ => track_handles.game.clone(),
            },
        ),
        false => {
            for track in &tracks {
                commands.entity(track).insert(FadeOut).remove::<FadeIn>();
            }
        }
    }
}
