use crate::GameState;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_lyon::plugin::Stage::Shape;
use bevy_prototype_lyon::prelude::shapes::*;
use bevy_prototype_lyon::prelude::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{random, Rng};

pub struct EntitiesPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct SpawnEntityStage;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_before(
            CoreStage::Update,
            SpawnEntityStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(spawn_entity.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(move_entities.system()),
        );
    }
}

#[derive(Clone)]
pub enum EntityForm {
    Rectangle,
    Pentagon,
    Hexagon,
}

impl EntityForm {
    pub fn to_shape(&self) -> RegularPolygon {
        let sides = match self {
            EntityForm::Rectangle => 4,
            EntityForm::Pentagon => 5,
            EntityForm::Hexagon => 6,
        };
        shapes::RegularPolygon {
            sides,
            feature: shapes::RegularPolygonFeature::Radius(30.0),
            ..shapes::RegularPolygon::default()
        }
    }
}

impl Distribution<EntityForm> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EntityForm {
        match rng.gen_range(0..3) {
            0 => EntityForm::Rectangle,
            1 => EntityForm::Pentagon,
            _ => EntityForm::Hexagon,
        }
    }
}

pub struct KnownEntity;

pub struct GameEntity {
    pub true_form: EntityForm,
    pub current_direction: Vec2,
    pub known: bool,
}

fn spawn_entity(mut commands: Commands) {
    let form: EntityForm = random();
    let entity = GameEntity {
        true_form: form.clone(),
        current_direction: Vec2::new((2. * random::<f32>()) - 1., (2. * random::<f32>()) - 1.)
            .normalize(),
        known: false,
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &Circle {
                radius: 30.,
                center: Default::default(),
            },
            ShapeColors {
                main: Color::MIDNIGHT_BLUE,
                outline: Color::ANTIQUE_WHITE,
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(0., 0., 0.)),
        ))
        .insert(entity);
}

fn move_entities(mut entities_query: Query<(&mut Transform, &GameEntity)>) {
    for (mut transform, entity) in entities_query.iter_mut() {
        transform.translation +=
            Vec3::new(entity.current_direction.x, entity.current_direction.y, 0.);
    }
}
