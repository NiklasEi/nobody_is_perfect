use crate::player::PlayerState;
use crate::GameState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_ui.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(update_courage.system()),
            );
    }
}

struct CourageMeter;
struct CourageMeterRest;

fn spawn_ui(mut commands: Commands, mut color_materials: ResMut<Assets<ColorMaterial>>) {
    let background = color_materials.add(Color::GRAY.into());
    let courage = color_materials.add(Color::ORANGE_RED.into());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(0.0), Val::Px(30.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(15.),
                    top: Val::Px(15.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: courage.clone(),
            ..Default::default()
        })
        .insert(CourageMeter);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(30.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(15.),
                    top: Val::Px(15.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: background.clone(),
            ..Default::default()
        })
        .insert(CourageMeterRest);
}

fn update_courage(
    mut courage: Query<&mut Style, (With<CourageMeter>, Without<CourageMeterRest>)>,
    mut courage_rest: Query<&mut Style, (With<CourageMeterRest>, Without<CourageMeter>)>,
    player_state: Res<PlayerState>,
) {
    for mut style in courage.iter_mut() {
        style.size = Size::new(Val::Px(player_state.courage * 2.), Val::Px(30.));
    }
    for mut style in courage_rest.iter_mut() {
        style.position = Rect {
            left: Val::Px(15. + player_state.courage * 2.),
            top: Val::Px(15.),
            ..Default::default()
        };
        style.size = Size::new(Val::Px(200. - player_state.courage * 2.), Val::Px(30.));
    }
}
