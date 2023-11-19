use bevy::input::gamepad::GamepadButtonChangedEvent;
use bevy::prelude::*;

use crate::asset_enum::{AssetDictionary, AssetEnumPlugin};
use crate::AppState;

use self::asset::{DialogAsset, DialogLoader};
pub use crate::dialog::dialogs::Dialog;
pub use crate::dialog::portrait::Portrait;

pub mod asset;
mod dialogs;
mod portrait;

pub struct DialogPlugin;

#[derive(Debug, Component)]
pub struct DialogSpeaker;

#[derive(Debug, Component)]
pub struct DialogText;

#[derive(Debug, Component)]
pub struct DialogUI;

#[derive(Debug, Component)]
pub struct DialogPortrait;

#[derive(Debug, Default, Resource)]
pub struct CurrentDialog {
    dialog: Dialog,
    current_line: usize,
}

impl CurrentDialog {
    pub fn set(&mut self, dialog: Dialog) {
        self.dialog = dialog;
        self.current_line = 0;
    }
}

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentDialog>()
            .add_plugins(AssetEnumPlugin::<Dialog, DialogAsset>::default())
            .add_plugins(AssetEnumPlugin::<Portrait, Image>::default())
            .init_asset::<DialogAsset>()
            .init_asset_loader::<DialogLoader>()
            .add_systems(Startup, setup_dialog)
            .add_systems(OnEnter(AppState::InDialog), set_visible::<DialogUI>)
            .add_systems(OnExit(AppState::InDialog), set_hidden::<DialogUI>)
            .add_systems(
                Update,
                (
                    update_dialog_text,
                    update_dialog_portrait,
                    dialog_input.run_if(in_state(AppState::InDialog)),
                ),
            );
    }
}

fn setup_dialog(mut commands: Commands) {
    // TODO: Figure out how to position and scale
    commands
        .spawn((
            NodeBundle {
                background_color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
                visibility: Visibility::Hidden,
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    right: Val::Px(140.0),
                    width: Val::Px(1000.0),
                    height: Val::Px(200.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            DialogUI,
        ))
        .with_children(|commands| {
            commands.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                DialogPortrait,
                DialogUI,
            ));
            commands
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    visibility: Visibility::Hidden,
                    ..Default::default()
                })
                .with_children(|commands| {
                    commands.spawn((
                        TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..Default::default()
                            },
                            visibility: Visibility::Hidden,
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font_size: 32.0,
                                    ..Default::default()
                                },
                            ),
                            ..Default::default()
                        }
                        .with_text_alignment(TextAlignment::Center),
                        DialogSpeaker,
                        DialogUI,
                    ));
                    commands.spawn((
                        TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(10.0)),
                                ..Default::default()
                            },
                            visibility: Visibility::Hidden,
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font_size: 24.0,
                                    ..Default::default()
                                },
                            ),
                            ..Default::default()
                        }
                        .with_text_alignment(TextAlignment::Center),
                        DialogText,
                        DialogUI,
                    ));
                });
        });
}

fn set_visible<T: Component>(mut dialog_vibility: Query<&mut Visibility, With<T>>) {
    for mut visibility in dialog_vibility.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn set_hidden<T: Component>(mut dialog_vibility: Query<&mut Visibility, With<T>>) {
    for mut visibility in dialog_vibility.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn dialog_input(
    mut current_dialog: ResMut<CurrentDialog>,
    mut events: EventReader<GamepadButtonChangedEvent>,
    mut state: ResMut<NextState<AppState>>,
    dialog_dict: Res<AssetDictionary<Dialog, DialogAsset>>,
    dialog_assets: Res<Assets<DialogAsset>>,
) {
    for event in events.read() {
        match event {
            GamepadButtonChangedEvent {
                button_type: GamepadButtonType::East,
                value,
                ..
            } if *value > 0.5 => {
                current_dialog.current_line += 1;
                if current_dialog.current_line
                    >= dialog_dict
                        .get(&current_dialog.dialog, &dialog_assets)
                        .map(|dialog| dialog.lines.len())
                        .unwrap_or(0)
                {
                    state.set(AppState::InGame);
                }
            }
            _ => {}
        }
    }
}

fn update_dialog_text(
    dialog: Res<CurrentDialog>,
    mut text_entity: Query<&mut Text, (With<DialogText>, Without<DialogSpeaker>)>,
    mut speaker_entity: Query<&mut Text, (With<DialogSpeaker>, Without<DialogText>)>,
    dialog_dict: Res<AssetDictionary<Dialog, DialogAsset>>,
    dialog_assets: Res<Assets<DialogAsset>>,
) {
    if dialog.is_changed() {
        let mut speaker_comp = speaker_entity.single_mut();
        let mut text_comp = text_entity.single_mut();
        if let Some(line) = dialog_dict
            .get(&dialog.dialog, &dialog_assets)
            .and_then(|dialog_asset| dialog_asset.lines.get(dialog.current_line))
        {
            text_comp.sections[0].value = line.text.to_string();
            speaker_comp.sections[0].value = line.speaker.to_string();
        }
    }
}
fn update_dialog_portrait(
    dialog: Res<CurrentDialog>,
    images: Res<AssetDictionary<Portrait, Image>>,
    mut image: Query<&mut UiImage, With<DialogPortrait>>,
    dialog_dict: Res<AssetDictionary<Dialog, DialogAsset>>,
    dialog_assets: Res<Assets<DialogAsset>>,
) {
    if dialog.is_changed() {
        if let Some(line) = dialog_dict
            .get(&dialog.dialog, &dialog_assets)
            .and_then(|dialog_asset| dialog_asset.lines.get(dialog.current_line))
        {
            image.single_mut().texture = images.get_handle(&line.portrait).unwrap_or_default();
        }
    }
}
