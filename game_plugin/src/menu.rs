use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{shapes, DrawMode, FillOptions, GeometryBuilder, ShapeColors};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(click_play_button.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(remove_menu.system()));
    }
}

struct Menu;

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

struct PlayButton;

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(40.0),
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
            Transform::from_translation(Vec3::new(0., 150., 20.)),
        ))
        .insert(Menu);
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .insert(PlayButton)
        .insert(Menu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });

    let mut menu_transform = Transform::from_translation(Vec3::new(50., -150., 10.));
    menu_transform.scale = Vec3::new(0.4, 0.4, 0.4);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_assets.menu.clone().into()),
            transform: menu_transform,
            ..Default::default()
        })
        .insert(Menu);
}

type ButtonInteraction<'a> = (
    Entity,
    &'a Interaction,
    &'a mut Handle<ColorMaterial>,
    &'a Children,
);

fn click_play_button(
    mut commands: Commands,
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                commands.entity(button).despawn();
                commands.entity(text).despawn();
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn remove_menu(mut commands: Commands, menu_query: Query<Entity, With<Menu>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
