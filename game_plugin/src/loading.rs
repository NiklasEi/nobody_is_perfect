mod paths;

use crate::loading::paths::PATHS;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_state.system()));
    }
}

pub struct LoadingState {
    textures: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,
    audio: Vec<HandleUntyped>,
}

pub struct FontAssets {
    pub fira_sans: Handle<Font>,
}

pub struct AudioAssets {
    pub background: Handle<AudioSource>,
    pub hi_1: Handle<AudioSource>,
    pub hi_2: Handle<AudioSource>,
    pub hi_3: Handle<AudioSource>,
    pub nope_1: Handle<AudioSource>,
    pub nope_2: Handle<AudioSource>,
    pub dying: Handle<AudioSource>,
    pub level_up: Handle<AudioSource>,
    pub lets_go: Handle<AudioSource>,
    pub won: Handle<AudioSource>,
}

pub struct TextureAssets {
    pub background: Handle<Texture>,
    pub menu: Handle<Texture>,
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.fira_sans));

    let mut audio: Vec<HandleUntyped> = vec![];
    audio.push(asset_server.load_untyped(PATHS.audio_background));
    audio.push(asset_server.load_untyped(PATHS.audio_hi_1));
    audio.push(asset_server.load_untyped(PATHS.audio_hi_2));
    audio.push(asset_server.load_untyped(PATHS.audio_hi_3));
    audio.push(asset_server.load_untyped(PATHS.audio_nope_1));
    audio.push(asset_server.load_untyped(PATHS.audio_nope_2));
    audio.push(asset_server.load_untyped(PATHS.audio_dying));
    audio.push(asset_server.load_untyped(PATHS.audio_level_up));
    audio.push(asset_server.load_untyped(PATHS.audio_lets_go));
    audio.push(asset_server.load_untyped(PATHS.audio_won));

    let mut textures: Vec<HandleUntyped> = vec![];
    textures.push(asset_server.load_untyped(PATHS.texture_background));
    textures.push(asset_server.load_untyped(PATHS.texture_menu));

    commands.insert_resource(LoadingState {
        textures,
        fonts,
        audio,
    });
}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.fonts.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.textures.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.audio.iter().map(|handle| handle.id))
    {
        return;
    }

    commands.insert_resource(FontAssets {
        fira_sans: asset_server.get_handle(PATHS.fira_sans),
    });

    commands.insert_resource(AudioAssets {
        background: asset_server.get_handle(PATHS.audio_background),
        hi_1: asset_server.get_handle(PATHS.audio_hi_1),
        hi_2: asset_server.get_handle(PATHS.audio_hi_2),
        hi_3: asset_server.get_handle(PATHS.audio_hi_3),
        nope_1: asset_server.get_handle(PATHS.audio_nope_1),
        nope_2: asset_server.get_handle(PATHS.audio_nope_2),
        dying: asset_server.get_handle(PATHS.audio_dying),
        level_up: asset_server.get_handle(PATHS.audio_level_up),
        lets_go: asset_server.get_handle(PATHS.audio_lets_go),
        won: asset_server.get_handle(PATHS.audio_won),
    });

    commands.insert_resource(TextureAssets {
        background: asset_server.get_handle(PATHS.texture_background),
        menu: asset_server.get_handle(PATHS.texture_menu),
    });

    state.set(GameState::RenderBackground).unwrap();
}
