use bevy::prelude::*;

use crate::{GameState};

use crate::render::button_systems::ButtonConfig;
use crate::start_server;

#[derive(Component)]
enum MenuButton {
    Host,
    Connect,
}

#[derive(Component)]
struct MainMenuEntity;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            setup_main_menu,
        );

        app.add_systems(
            Update,
            menu_button_system,
        );

        app.add_systems(
            OnExit(GameState::MainMenu),
            cleanup_main_menu,
        );
    }
}

fn setup_main_menu(
    mut commands: Commands,
) {
    commands.spawn((Camera2d, MainMenuEntity));

    commands.spawn((
        MainMenuEntity,
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(20),
            ..default()
        },
        children![
            (
                Text::new("Strategy Game"),
            ),

            (
                Button,
                ButtonConfig {
                    hover: true,
                },
                MenuButton::Host,
                Node {
                    width: px(250),
                    height: px(60),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("Server starten"),
                    ),
                ],
            ),

            (
                Button,
                ButtonConfig {
                    hover: true,
                },
                MenuButton::Connect,
                Node {
                    width: px(250),
                    height: px(60),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("Verbinden"),
                    ),
                ],
            ),
        ],
    ));
}

fn menu_button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction, 
            &mut MenuButton, 
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut button) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match *button {
            MenuButton::Host => {
                println!("Server starten");
                start_server();
            }

            MenuButton::Connect => {
                println!("Mit Server verbinden");
                next_state.set(GameState::InGame);
            }
        }
    }
}

fn cleanup_main_menu(
    mut commands: Commands,
    entities: Query<Entity, With<MainMenuEntity>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}