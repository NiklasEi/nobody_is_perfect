use bevy::prelude::*;
use crate::actions::Actions;
use crate::{GameState, GameWorld};
use bevy_prototype_lyon::prelude::{FillOptions, GeometryBuilder, Geometry, PathBuilder, StrokeOptions};
use bevy_prototype_lyon::entity::ShapeColors;
use bevy_prototype_lyon::utils::DrawMode;
use bevy_prototype_lyon::shapes;
use std::f32::consts::PI;

pub struct PlayerPlugin;

pub struct Player;
pub struct PlayerCamera;
pub struct  FieldOfView;

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
enum PlayerSystemLabels {
    MovePlayer
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(CursorPosition {
            position: Vec2::new(0., 0.)
        }).add_system_set(SystemSet::on_enter(GameState::Playing)
            .with_system(spawn_player.system())
            .with_system(spawn_field_of_view.system())
            .with_system(spawn_camera.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(cursor_grab_system.system())
                    .with_system(move_player.system().label(PlayerSystemLabels::MovePlayer))
                    .with_system(move_field_of_view.system().after(PlayerSystemLabels::MovePlayer)),
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

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    button: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if button.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
    }
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
            Transform::from_translation(Vec3::new(0., 0., 1.)),
        ))
        .insert(Player);
}

pub fn field_of_view() -> impl Geometry {
    let mut builder = PathBuilder::new();
    builder.move_to(Vec2::ZERO);
    builder.line_to(Vec2::new(-35., 150.));
    builder.arc(Vec2::new(-0., 150.), Vec2::new(35., 35.), -PI, -PI);
    builder.line_to(Vec2::ZERO);
    builder.build()
}

fn spawn_field_of_view(
    mut commands: Commands,
) {
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &field_of_view(),
            ShapeColors {
                main: Color::GRAY,
                outline: Color::ANTIQUE_WHITE
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(0., 0., 2.)),
        ))
        .insert(FieldOfView);
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

fn move_field_of_view(player_query: Query<&Transform,(With<Player>, Without<FieldOfView>)>, mut field_of_view_query: Query<&mut Transform,( With<FieldOfView>, Without<Player>)>) {
    for player_transform in player_query.iter() {
        for mut fov_transform in field_of_view_query.iter_mut() {
            fov_transform.translation = Vec3::new(player_transform.translation.x, player_transform.translation.y, 0.);
            fov_transform.rotation = player_transform.rotation;
        }
    }
}

fn remove_player(mut commands: Commands, player_query: Query<Entity, With<Player>>, mut windows: ResMut<Windows>,) {
    for player in player_query.iter() {
        commands.entity(player).despawn();
    }

    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(false);
}
