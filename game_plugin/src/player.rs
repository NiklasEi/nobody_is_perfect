use bevy::prelude::*;
use crate::actions::Actions;
use crate::{GameState, GameWorld};
use bevy_prototype_lyon::prelude::{FillOptions, GeometryBuilder};
use bevy_prototype_lyon::entity::ShapeColors;
use bevy_prototype_lyon::utils::DrawMode;
use bevy_prototype_lyon::shapes;

pub struct PlayerPlugin;

pub struct Player;
pub struct PlayerCamera;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(CursorPosition {
            position: Vec2::new(0., 0.)
        }).add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player.system()).with_system(spawn_camera.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(remove_player.system()));

    }
}

struct CursorPosition {
    position: Vec2
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}


fn spawn_player(
    mut commands: Commands,
) {
    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(30.0),
        ..shapes::RegularPolygon::default()
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors {
                main: Color::MIDNIGHT_BLUE,
                outline: Color::ANTIQUE_WHITE
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(0., 0., 0.)),
        ))
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    world: Res<GameWorld>,
    windows: Res<Windows>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut cursor_position: ResMut<CursorPosition>,
    mut player_query: Query<&mut Transform,( With<Player>, Without<PlayerCamera>)>,
    mut player_camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let speed = 150.;
    let movement = if let Some(player_movement) = actions.player_movement {
        Vec3::new(
            player_movement.x * speed * time.delta_seconds(),
            player_movement.y * speed * time.delta_seconds(),
            0.,
        )
    } else {
        Vec3::ZERO
    };
    if let Some(new_cursor_position) = cursor_moved.iter().last() {
        let window = windows.get(new_cursor_position.id).unwrap();
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        cursor_position.position = new_cursor_position.position - size / 2.0;
    };
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
        player_transform.translation.x = player_transform.translation.x.clamp(-world.border, world.border);
        player_transform.translation.y = player_transform.translation.y.clamp(-world.border, world.border);
        player_transform.rotation = Quat::from_rotation_z(-cursor_position.position.angle_between(Vec2::new(0., 1.)));
            for mut player_camera_transform in player_camera_query.iter_mut() {
            player_camera_transform.translation = player_transform.translation;
        }
    }
}

fn remove_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for player in player_query.iter() {
        commands.entity(player).despawn();
    }
}
