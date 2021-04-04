// disable console opening on windows
#![windows_subsystem = "windows"]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use game_plugin::GamePlugin;

fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::GRAY))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Nobody is perfect".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
