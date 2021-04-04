use crate::actions::Actions;
use crate::entities::{BefriendedEntity, GameEntity};
use crate::{GameState, GameWorld};
use bevy::ecs::component::{ComponentDescriptor, StorageType};
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeColors;
use bevy_prototype_lyon::prelude::{FillOptions, Geometry, GeometryBuilder, PathBuilder};
use bevy_prototype_lyon::shapes;
use bevy_prototype_lyon::utils::DrawMode;
use std::f32::consts::PI;

pub struct PlayerPlugin;

pub struct Player;

pub struct PlayerState {
    pub level: usize,
    pub courage: f32,
    pub dead: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            dead: false,
            level: 0,
            courage: 50.0,
        }
    }
}

pub struct PlayerCamera;

pub struct FieldOfView {
    half_angle: f32,
    height: f32,
}
pub struct InFieldOfView;

pub struct BefriendEvent;
pub struct DyingEvent;
pub struct LevelUpEvent;
pub struct NopeEvent;

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
enum PlayerSystemLabels {
    MovePlayer,
    MoveFieldOfView,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(CursorPosition {
            position: Vec2::new(0., 0.),
        })
        .register_component(ComponentDescriptor::new::<InFieldOfView>(
            StorageType::SparseSet,
        ))
        .add_event::<BefriendEvent>()
        .add_event::<NopeEvent>()
        .add_event::<DyingEvent>()
        .add_event::<LevelUpEvent>()
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system())
                .with_system(spawn_field_of_view.system())
                .with_system(spawn_camera.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player.system().label(PlayerSystemLabels::MovePlayer))
                .with_system(
                    move_field_of_view
                        .system()
                        .label(PlayerSystemLabels::MoveFieldOfView)
                        .after(PlayerSystemLabels::MovePlayer),
                )
                .with_system(
                    mark_entities_in_field_of_view
                        .system()
                        .after(PlayerSystemLabels::MoveFieldOfView),
                ).with_system(remove_fov_on_death.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(remove_player.system()));
    }
}

struct CursorPosition {
    position: Vec2,
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}

fn spawn_player(mut commands: Commands) {
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
                outline: Color::ANTIQUE_WHITE,
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
        ))
        .insert(Player);
    commands.insert_resource(PlayerState::default());
}

pub fn build_fov_geometry(field_of_view: &FieldOfView) -> impl Geometry {
    let mut builder = PathBuilder::new();
    let radius = field_of_view.height * field_of_view.half_angle.sin();
    let left = Vec2::new(
        -radius,
        field_of_view.height * field_of_view.half_angle.cos(),
    );
    builder.move_to(Vec2::ZERO);
    builder.line_to(left);
    builder.arc(
        Vec2::new(0., 0.),
        Vec2::new(field_of_view.height, field_of_view.height),
        -field_of_view.half_angle * 2.,
        -PI,
    );
    builder.line_to(Vec2::ZERO);
    builder.build()
}

fn spawn_field_of_view(mut commands: Commands) {
    let field_of_view = FieldOfView {
        half_angle: PI / 10.,
        height: 150.,
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &build_fov_geometry(&field_of_view),
            ShapeColors {
                main: Color::GRAY,
                outline: Color::ANTIQUE_WHITE,
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(0., 0., 0.)),
        ))
        .insert(field_of_view);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    world: Res<GameWorld>,
    windows: Res<Windows>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut cursor_position: ResMut<CursorPosition>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
    mut player_camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player_state: Res<PlayerState>,
) {
    if player_state.dead {
        for mut player_transform in player_query.iter_mut() {
            player_transform.scale = player_transform.scale * 0.99;
        }
        return;
    }
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
        player_transform.translation.x = player_transform
            .translation
            .x
            .clamp(-world.border, world.border);
        player_transform.translation.y = player_transform
            .translation
            .y
            .clamp(-world.border, world.border);
        player_transform.rotation =
            Quat::from_rotation_z(-cursor_position.position.angle_between(Vec2::new(0., 1.)));
        for mut player_camera_transform in player_camera_query.iter_mut() {
            player_camera_transform.translation = player_transform.translation;
        }
    }
}

fn move_field_of_view(
    player_query: Query<&Transform, (With<Player>, Without<FieldOfView>)>,
    mut field_of_view_query: Query<&mut Transform, (With<FieldOfView>, Without<Player>)>,
    player_state: Res<PlayerState>,
) {
    if player_state.dead {
        return;
    }
    for player_transform in player_query.iter() {
        for mut fov_transform in field_of_view_query.iter_mut() {
            fov_transform.translation = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                0.,
            );
            fov_transform.rotation = player_transform.rotation;
        }
    }
}

fn mark_entities_in_field_of_view(
    mut commands: Commands,
    field_of_view: Query<(&Transform, &FieldOfView), Without<GameEntity>>,
    mut entities: Query<(Entity, &Transform, &mut GameEntity), Without<BefriendedEntity>>,
    mut befriend_event: EventWriter<BefriendEvent>,
    mut nope_event: EventWriter<NopeEvent>,
    mut level_up_event: EventWriter<LevelUpEvent>,
    mut die_event: EventWriter<DyingEvent>,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
) {
    if player_state.dead {
        return;
    }
    if let Ok((fov_transform, field_of_view)) = field_of_view.single() {
        let player_rotation = fov_transform
            .rotation
            .angle_between(Quat::from_rotation_z(0.));
        let player_rotation_side = fov_transform
            .rotation
            .angle_between(Quat::from_rotation_z(PI / 2.));
        let fov_direction = Vec3::new(
            if player_rotation_side > PI / 2. {
                player_rotation.sin()
            } else {
                -player_rotation.sin()
            },
            player_rotation.cos(),
            0.,
        );
        let millis_since_startup = time.time_since_startup().as_millis();
        for (entity, transform, mut game_entity) in entities.iter_mut() {
            let entity_from_player = transform.translation - fov_transform.translation;
            if (entity_from_player.length() < field_of_view.height)
                && (entity_from_player.angle_between(fov_direction).abs()
                    < field_of_view.half_angle)
            {
                if game_entity.true_form.level() <= player_state.level {
                    befriend_event.send(BefriendEvent);
                    let new_game_entity = GameEntity {
                        true_form: game_entity.true_form.clone(),
                        current_direction: game_entity.current_direction.clone(),
                        last_contact: game_entity.last_contact,
                        next_direction_change: game_entity.next_direction_change,
                        known: true,
                    };
                    commands.entity(entity).despawn();
                    commands
                        .spawn_bundle(GeometryBuilder::build_as(
                            &new_game_entity.true_form.to_shape(),
                            ShapeColors {
                                main: Color::GREEN,
                                outline: Color::ANTIQUE_WHITE,
                            },
                            DrawMode::Fill(FillOptions::default()),
                            transform.clone(),
                        ))
                        .insert(new_game_entity)
                        .insert(BefriendedEntity);
                    player_state.courage += 20.;
                } else if millis_since_startup - game_entity.last_contact.as_millis() > 2000 {
                    player_state.courage -= 20.;
                    if player_state.courage > 0.1 {
                        nope_event.send(NopeEvent);
                    }
                    game_entity.last_contact = time.time_since_startup();
                }
            }
        }
        player_state.courage = player_state.courage.clamp(0., 100.);
        if player_state.courage > 99.5 {
            player_state.courage = 0.;
            player_state.level += 1;
            level_up_event.send(LevelUpEvent);
        } else if player_state.courage < 0.1 {
            player_state.dead = true;
            die_event.send(DyingEvent);
        }
    }
}

fn remove_fov_on_death(
    mut commands: Commands,
    mut events: EventReader<DyingEvent>,
    fov_query: Query<Entity, With<FieldOfView>>,
) {
    if let Some(_event) = events.iter().last() {
        for fov in fov_query.iter() {
            commands.entity(fov).despawn();
        }
    }
}


fn remove_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    fov_query: Query<Entity, With<FieldOfView>>,
) {
    for player in player_query.iter() {
        commands.entity(player).despawn();
    }
    for fov in fov_query.iter() {
        commands.entity(fov).despawn();
    }
}
