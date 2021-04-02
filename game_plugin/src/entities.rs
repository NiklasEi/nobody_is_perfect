use bevy::prelude::*;
use rand::prelude::Distribution;
use rand::distributions::Standard;
use rand::{Rng, random};
use bevy::core::FixedTimestep;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::prelude::shapes::*;
use crate::GameState;

pub struct EntitiesPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct SpawnEntityStage;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_before(
            CoreStage::Update,
            SpawnEntityStage,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(2.0)
                )
                .with_system(spawn_entity.system())
        ).add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_entities.system()));
    }
}

#[derive(Clone)]
enum EntityForm {
    Rectangle,
    Pentagon,
    Hexagon
}

impl EntityForm {
    fn to_shape(&self) -> RegularPolygon {
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

struct Entity {
    true_form: EntityForm,
    current_direction: Vec2
}

fn spawn_entity(mut commands: Commands) {
    let form: EntityForm = random();
    let entity = Entity {
        true_form: form.clone(),
        current_direction: Vec2::new((2. * random::<f32>()) - 1.,(2. * random::<f32>()) - 1.).normalize()
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &form.to_shape(),
            ShapeColors {
                main: Color::MIDNIGHT_BLUE,
                outline: Color::ANTIQUE_WHITE
            },
            DrawMode::Fill(FillOptions::default()),
            Transform::from_translation(Vec3::new(0., 0., 0.)),
        ))
        .insert(entity);
}

fn move_entities(mut entities_query: Query<(&mut Transform, &Entity)>) {
    for (mut transform, entity) in entities_query.iter_mut() {
        transform.translation += Vec3::new(entity.current_direction.x, entity.current_direction.y, 0.);
    }
}
