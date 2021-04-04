use crate::loading::FontAssets;
use crate::player::{DyingEvent, LevelUpEvent, PlayerState};
use crate::GameState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_ui.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_courage.system())
                    .with_system(spawn_death_ui.system())
                    .with_system(update_courage_level.system())
                    .with_system(click_retry_button.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(remove_ui.system()));
    }
}

struct Ui;
struct CourageMeter;
struct CourageMeterRest;
struct RetryButton;
struct CourageLevel;

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

fn spawn_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
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
        .insert(CourageMeter)
        .insert(Ui);
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
        .insert(CourageMeterRest)
        .insert(Ui);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(30.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(15.),
                    top: Val::Px(15.),
                    ..Default::default()
                },
                padding: {
                    Rect {
                        left: Val::Px(5.),
                        ..Default::default()
                    }
                },
                ..Default::default()
            },
            material: background.clone(),
            ..Default::default()
        })
        .insert(Ui)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Courage level: 1/7".to_string(),
                            style: TextStyle {
                                font_size: 30.0,
                                color: Color::BLACK,
                                font: font_assets.fira_sans.clone(),
                                ..Default::default()
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(CourageLevel);
        });
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

fn update_courage_level(
    mut courage_level: Query<&mut Text, With<CourageLevel>>,
    mut level_up_events: EventReader<LevelUpEvent>,
    player_state: Res<PlayerState>,
) {
    if let Some(_event) = level_up_events.iter().last() {
        for mut text in courage_level.iter_mut() {
            text.sections.first_mut().unwrap().value =
                format!("Courage level: {}/7", player_state.level + 1)
        }
    }
}

fn spawn_death_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_materials: Res<ButtonMaterials>,
    mut events: EventReader<DyingEvent>,
) {
    if let Some(_event) = events.iter().last() {
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: Rect {
                        right: Val::Auto,
                        left: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Percent(20.),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .insert(RetryButton)
            .insert(Ui)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Restart".to_string(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                font: font_assets.fira_sans.clone(),
                                ..Default::default()
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                });
            });
    }
}

fn click_retry_button(
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Restart).unwrap();
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

fn remove_ui(mut commands: Commands, text_query: Query<Entity, With<Ui>>) {
    for entity in text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
