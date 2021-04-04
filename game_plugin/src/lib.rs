mod actions;
mod audio;
mod entities;
mod loading;
mod menu;
mod player;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::{LoadingPlugin, TextureAssets};
use crate::player::{PlayerCamera, PlayerPlugin};

use bevy::app::AppBuilder;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use crate::entities::EntitiesPlugin;
use crate::menu::MenuPlugin;
use crate::ui::UiPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    RenderBackground,
    Menu,
    Restart,
}

pub struct GamePlugin;

pub struct GameWorld {
    border: f32,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .insert_resource(GameWorld { border: 1000. })
            .add_plugin(ShapePlugin)
            .add_plugin(EntitiesPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Restart).with_system(switch_to_game.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::RenderBackground)
                    .with_system(spawn_camera_and_background.system()),
            );
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
    }
}

fn switch_to_game(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Playing).unwrap();
}

fn spawn_camera_and_background(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut state: ResMut<State<GameState>>,
    texture_assets: Res<TextureAssets>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_assets.texture_background.clone().into()),
        ..Default::default()
    });
    state.set(GameState::Menu).unwrap();
}
