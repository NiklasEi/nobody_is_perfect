mod actions;
mod audio;
mod entities;
mod loading;
mod player;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::player::PlayerPlugin;

use bevy::app::AppBuilder;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use crate::entities::EntitiesPlugin;
use crate::ui::UiPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
}

pub struct GamePlugin;

pub struct GameWorld {
    border: f32,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .insert_resource(GameWorld {
                border: 500.,
            })
            .add_plugin(ShapePlugin)
            .add_plugin(EntitiesPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::default())
            ;
    }
}
