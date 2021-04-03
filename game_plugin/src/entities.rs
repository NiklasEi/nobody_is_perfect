use crate::player::{LevelUpEvent, PlayerState};
use crate::GameState;
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
        app.insert_resource(EntityTimer::from_seconds(2., true))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_entities.system())
                    .with_system(redraw_after_level_up.system())
                    .with_system(spawn_entity.system()),
            );
    }
}

type EntityTimer = Timer;

#[derive(Clone)]
pub enum EntityForm {
    Rectangle,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
    Nonagon,
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

fn spawn_entity(
    mut commands: Commands,
    player_state: Res<PlayerState>,
    mut timer: ResMut<EntityTimer>,
    time: Res<Time>,
) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let form: EntityForm = random();
    let entity = GameEntity {
        true_form: form.clone(),
        current_direction: Vec2::new((2. * random::<f32>()) - 1., (2. * random::<f32>()) - 1.)
            .normalize(),
        last_contact: Duration::from_secs(0),
        next_direction_change: time.time_since_startup() + Duration::from_secs(2),
        known: false,
    };
    if player_state.level >= entity.true_form.level() {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &entity.true_form.to_shape(),
                ShapeColors {
                    main: Color::DARK_GRAY,
                    outline: Color::ANTIQUE_WHITE,
                },
                DrawMode::Fill(FillOptions::default()),
                Transform::from_translation(Vec3::new(0., 0., 0.)),
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
                Transform::from_translation(Vec3::new(0., 0., 0.)),
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
                            main: Color::DARK_GRAY,
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

fn move_entities(mut entities_query: Query<(&mut Transform, &mut GameEntity)>, time: Res<Time>) {
    let millis_since_startup = time.time_since_startup().as_millis();
    let mut random = thread_rng();
    for (mut transform, mut game_entity) in entities_query.iter_mut() {
        if millis_since_startup >= game_entity.next_direction_change.as_millis() {
            game_entity.current_direction = Vec2::new(
                (2. * random.gen::<f32>()) - 1.,
                (2. * random.gen::<f32>()) - 1.,
            )
            .normalize();
            game_entity.next_direction_change = time.time_since_startup()
                + Duration::from_secs(2)
                + Duration::from_secs(3).mul_f32(random.gen::<f32>());
        }
        transform.translation += Vec3::new(
            game_entity.current_direction.x,
            game_entity.current_direction.y,
            0.,
        );
    }
}
