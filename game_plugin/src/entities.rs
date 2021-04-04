use crate::player::{LevelUpEvent, PlayerState};
use crate::{GameState, GameWorld};
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_prototype_lyon::prelude::shapes::*;
use bevy_prototype_lyon::prelude::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{random, thread_rng, Rng};

pub struct EntitiesPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct SpawnEntityStage;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(EntityTimer::from_seconds(2.2, true))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_beginning_entities.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_entities.system())
                    .with_system(redraw_after_level_up.system())
                    .with_system(spawn_entity.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(remove_entities.system()),
            );
    }
}

type EntityTimer = Timer;

pub enum Spawn {
    UpLeft,
    UpRight,
    BottomLeft,
    BottomRight,
}

impl Spawn {
    pub fn get_position(&self) -> Vec2 {
        match self {
            Spawn::UpLeft => Vec2::new(250., -250.),
            Spawn::UpRight => Vec2::new(250., 250.),
            Spawn::BottomLeft => Vec2::new(-250., -250.),
            Spawn::BottomRight => Vec2::new(-250., 250.),
        }
    }
}

impl Distribution<Spawn> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Spawn {
        match rng.gen_range(0..4) {
            0 => Spawn::UpLeft,
            1 => Spawn::UpRight,
            2 => Spawn::BottomLeft,
            _ => Spawn::BottomRight,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum EntityForm {
    Rectangle,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
    Nonagon,
    Enemy,
}

pub fn build_enemy_geometry() -> impl Geometry {
    let mut builder = PathBuilder::new();
    builder.line_to(Vec2::new(-20., 20.));
    builder.line_to(Vec2::new(20., 20.));
    builder.line_to(Vec2::new(20., -20.));
    builder.line_to(Vec2::new(-20., -20.));
    builder.line_to(Vec2::new(-20., 15.));
    builder.line_to(Vec2::new(15., 15.));
    builder.line_to(Vec2::new(15., -15.));
    builder.line_to(Vec2::new(-15., -15.));
    builder.line_to(Vec2::new(-15., 10.));
    builder.line_to(Vec2::new(10., 10.));
    builder.line_to(Vec2::new(10., -10.));
    builder.line_to(Vec2::new(-10., -10.));
    builder.line_to(Vec2::new(-10., 5.));
    builder.line_to(Vec2::new(5., 5.));
    builder.line_to(Vec2::new(5., -5.));
    builder.line_to(Vec2::new(-5., -5.));
    builder.line_to(Vec2::new(-5., 0.));
    builder.line_to(Vec2::ZERO);
    builder.build()
}

impl EntityForm {
    pub fn to_shape(&self) -> RegularPolygon {
        let sides = match self {
            EntityForm::Rectangle => 4,
            EntityForm::Pentagon => 5,
            EntityForm::Hexagon => 6,
            EntityForm::Heptagon => 7,
            EntityForm::Octagon => 8,
            EntityForm::Nonagon => 9,
            _ => 0,
        };
        shapes::RegularPolygon {
            sides,
            feature: shapes::RegularPolygonFeature::Radius(30.0),
            ..shapes::RegularPolygon::default()
        }
    }

    pub fn level(&self) -> usize {
        match self {
            EntityForm::Rectangle => 0,
            EntityForm::Pentagon => 1,
            EntityForm::Hexagon => 2,
            EntityForm::Heptagon => 3,
            EntityForm::Octagon => 4,
            EntityForm::Nonagon => 5,
            EntityForm::Enemy => 99,
        }
    }
}

impl Distribution<EntityForm> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EntityForm {
        match rng.gen_range(0..6) {
            0 => EntityForm::Rectangle,
            1 => EntityForm::Pentagon,
            2 => EntityForm::Hexagon,
            3 => EntityForm::Heptagon,
            4 => EntityForm::Octagon,
            _ => EntityForm::Nonagon,
        }
    }
}

pub struct BefriendedEntity;

#[derive(Clone)]
pub struct GameEntity {
    pub true_form: EntityForm,
    pub current_direction: Vec2,
    pub last_contact: Duration,
    pub next_direction_change: Duration,
    pub known: bool,
}

fn spawn_beginning_entities(mut commands: Commands) {
    for count in 0..30 {
        let form: EntityForm = if count < 5 {
            EntityForm::Rectangle
        } else if count < 10 {
            EntityForm::Pentagon
        } else {
            random()
        };
        let entity = GameEntity {
            true_form: form.clone(),
            current_direction: Vec2::new((2. * random::<f32>()) - 1., (2. * random::<f32>()) - 1.)
                .normalize(),
            last_contact: Duration::from_secs(0),
            next_direction_change: Duration::from_secs(2)
                + Duration::from_secs(3).mul_f32(random::<f32>()),
            known: false,
        };
        let position = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5).normalize() * 500.;
        if entity.true_form.level() == 0 {
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &entity.true_form.to_shape(),
                    ShapeColors {
                        main: Color::AQUAMARINE,
                        outline: Color::ANTIQUE_WHITE,
                    },
                    DrawMode::Fill(FillOptions::default()),
                    Transform::from_translation(Vec3::new(position.x, position.y, 10.)),
                ))
                .insert(entity);
        } else {
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &Circle {
                        radius: 26.,
                        center: Default::default(),
                    },
                    ShapeColors {
                        main: Color::DARK_GRAY,
                        outline: Color::ANTIQUE_WHITE,
                    },
                    DrawMode::Fill(FillOptions::default()),
                    Transform::from_translation(Vec3::new(position.x, position.y, 10.)),
                ))
                .insert(entity);
        }
    }
}

fn spawn_entity(
    mut commands: Commands,
    player_state: Res<PlayerState>,
    mut timer: ResMut<EntityTimer>,
    time: Res<Time>,
) {
    if player_state.level > 5 || player_state.dead {
        return;
    }
    if !timer.tick(time.delta()).just_finished() {
        return;
    }
    let spawn: Spawn = random();
    let position = spawn.get_position();
    let entity = GameEntity {
        true_form: if random::<f32>() > 0.8 {
            random()
        } else {
            EntityForm::Enemy
        },
        current_direction: Vec2::new((2. * random::<f32>()) - 1., (2. * random::<f32>()) - 1.)
            .normalize(),
        last_contact: time.time_since_startup(),
        next_direction_change: time.time_since_startup() + Duration::from_secs(2),
        known: false,
    };
    let draw_mode = if entity.true_form == EntityForm::Enemy {
        DrawMode::Stroke(
            StrokeOptions::default()
                .with_line_join(LineJoin::Round)
                .with_line_width(3.),
        )
    } else {
        DrawMode::Fill(FillOptions::default())
    };
    if entity.true_form == EntityForm::Enemy {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &build_enemy_geometry(),
                ShapeColors {
                    main: Color::DARK_GRAY,
                    outline: Color::ANTIQUE_WHITE,
                },
                draw_mode,
                Transform::from_translation(Vec3::new(position.x, position.y, 10.)),
            ))
            .insert(entity);
        return;
    }
    if player_state.level >= entity.true_form.level() {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &entity.true_form.to_shape(),
                ShapeColors {
                    main: Color::AQUAMARINE,
                    outline: Color::ANTIQUE_WHITE,
                },
                draw_mode,
                Transform::from_translation(Vec3::new(position.x, position.y, 10.)),
            ))
            .insert(entity);
    } else {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &Circle {
                    radius: 26.,
                    center: Default::default(),
                },
                ShapeColors {
                    main: Color::DARK_GRAY,
                    outline: Color::ANTIQUE_WHITE,
                },
                draw_mode,
                Transform::from_translation(Vec3::new(position.x, position.y, 10.)),
            ))
            .insert(entity);
    }
}

fn redraw_after_level_up(
    mut commands: Commands,
    mut level_up_events: EventReader<LevelUpEvent>,
    player_state: Res<PlayerState>,
    entities: Query<(Entity, &Transform, &GameEntity), Without<BefriendedEntity>>,
) {
    if let Some(_event) = level_up_events.iter().last() {
        for (entity, transform, game_entity) in entities.iter() {
            if game_entity.true_form.level() == player_state.level {
                commands.entity(entity).despawn();
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &game_entity.true_form.to_shape(),
                        ShapeColors {
                            main: Color::AQUAMARINE,
                            outline: Color::ANTIQUE_WHITE,
                        },
                        DrawMode::Fill(FillOptions::default()),
                        transform.clone(),
                    ))
                    .insert(game_entity.clone());
            }
        }
    }
}

fn move_entities(
    mut entities_query: Query<(&mut Transform, &mut GameEntity)>,
    time: Res<Time>,
    game_world: Res<GameWorld>,
) {
    let millis_since_startup = time.time_since_startup().as_millis();
    let mut random = thread_rng();
    for (mut transform, mut game_entity) in entities_query.iter_mut() {
        transform.translation += Vec3::new(
            game_entity.current_direction.x * time.delta_seconds() * 100.,
            game_entity.current_direction.y * time.delta_seconds() * 100.,
            0.,
        );
        let mut change_direction = false;
        if transform.translation.x > game_world.border
            || transform.translation.x < -game_world.border
            || transform.translation.y > game_world.border
            || transform.translation.y < -game_world.border
        {
            transform.translation -= Vec3::new(
                game_entity.current_direction.x * time.delta_seconds() * 100.,
                game_entity.current_direction.y * time.delta_seconds() * 100.,
                0.,
            );
            change_direction = true;
        }
        if millis_since_startup >= game_entity.next_direction_change.as_millis() || change_direction
        {
            game_entity.current_direction = Vec2::new(
                (2. * random.gen::<f32>()) - 1.,
                (2. * random.gen::<f32>()) - 1.,
            )
            .normalize();
            game_entity.next_direction_change = time.time_since_startup()
                + Duration::from_secs(2)
                + Duration::from_secs(3).mul_f32(random.gen::<f32>());
        }
    }
}

fn remove_entities(mut commands: Commands, entity_query: Query<Entity, With<GameEntity>>) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}
