use bevy::{prelude::*, ui_widgets::ScrollArea};
use rand::RngExt;
use shared::ClientMessage;

use crate::render::button_systems::ButtonConfig;
use crate::{GameSet, GameState, Rnd};
use bevy::ui::ScrollPosition;

#[derive(Component)]
pub enum GUIButton {
    Politics,
    Production,
    NationView,
    Treasury,
    Military,
    Research,
    Pause,
    Speed(usize),
    Notification(Entity),
}

#[derive(Component)]
pub struct DateText;

#[derive(Component)]
pub struct NotificationList;

#[derive(Component)]
pub struct NotificationItem {
    pub title: String,
    pub content: String,
    pub open: bool,
}

#[derive(Component)]
pub struct TimeButton {
    pub id: usize,
}

#[derive(Component)]
pub struct SubMenu;

#[derive(Component)]
pub struct SubMenuCloseButton;

#[derive(Component)]
struct GUIEntity;

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_gui);

        app.add_systems(Update, ((gui_button_system, submenu_close_system)).in_set(GameSet));

        app.add_systems(OnExit(GameState::InGame), cleanup_gui);
    }
}

fn setup_gui(mut commands: Commands, asset_server: Res<AssetServer>, mut rnd: ResMut<Rnd>) {
    let flag = asset_server.load(format!(
        "gfx/flags/flag{}.png",
        rnd.rng_object.random_range(1..=6)
    ));

    commands.spawn((
        GUIEntity,
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            // ============================================================
            // TOP BAR
            // ============================================================
            (
                Node {
                    width: percent(100),
                    height: px(80),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::horizontal(px(10)),
                    column_gap: px(10),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.06, 0.06, 0.06)),
                BorderColor {
                    bottom: Color::WHITE,
                    ..default()
                },
                children![
                    (
                        Node {
                            height: px(80),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: px(10),
                            ..default()
                        },
                        children![
                            // FLAG
                            (
                                Button,
                                ButtonConfig { hover: false },
                                GUIButton::NationView,
                                Node {
                                    height: px(60),
                                    aspect_ratio: Some(1.5),
                                    border: UiRect::all(px(2)),
                                    border_radius: px(3).into(),
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                ImageNode {
                                    image: flag.clone(),
                                    image_mode: NodeImageMode::Stretch,
                                    ..default()
                                },
                            ),
                            // NATION NAME
                            (
                                Node {
                                    width: px(180),
                                    height: px(60),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                children![
                                    (
                                        Text::new("WEIMAR REPUBLIC"),
                                        TextFont {
                                            font_size: px(18).into(),
                                            ..default()
                                        },
                                    ),
                                    (
                                        Text::new("Zentrum & SPD & DVP"),
                                        TextFont {
                                            font_size: px(14).into(),
                                            ..default()
                                        },
                                    ),
                                ],
                            ),
                            // MONEY
                            (
                                Node {
                                    width: px(120),
                                    height: px(45),
                                    border: UiRect::all(px(2)),
                                    border_radius: px(3).into(),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![
                                    (
                                        ImageNode::new(asset_server.load("gfx/menu/budget.png")),
                                        Node {
                                            height: percent(100),
                                            aspect_ratio: Some(1.),
                                            ..default()
                                        }
                                    ),
                                    (Text::new("12.4K"),),
                                ],
                            ),
                            // POPULATION
                            (
                                Node {
                                    width: px(120),
                                    height: px(45),
                                    border: UiRect::all(px(2)),
                                    border_radius: px(3).into(),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![
                                    (
                                        ImageNode::new(
                                            asset_server.load("gfx/menu/population.png")
                                        ),
                                        Node {
                                            height: percent(100),
                                            aspect_ratio: Some(1.),
                                            ..default()
                                        }
                                    ),
                                    (Text::new("24.8M"),),
                                ],
                            ),
                            // POLITICAL POWER
                            (
                                Node {
                                    width: px(120),
                                    height: px(45),
                                    border: UiRect::all(px(2)),
                                    border_radius: px(3).into(),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![
                                    (
                                        ImageNode::new(
                                            asset_server.load("gfx/menu/political_power.png")
                                        ),
                                        Node {
                                            height: percent(100),
                                            aspect_ratio: Some(1.),
                                            ..default()
                                        }
                                    ),
                                    (Text::new("145"),),
                                ],
                            ),
                        ],
                    ),
                    (
                        Node {
                            height: px(60),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: px(5),
                            ..default()
                        },
                        children![
                            // DATUM
                            (
                                Node {
                                    width: px(120),
                                    height: px(40),
                                    border: UiRect::all(px(2)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(
                                    DateText,
                                    Text::new("--"),
                                    TextFont {
                                        font_size: px(16).into(),
                                        ..default()
                                    },
                                ),],
                            ),
                            // PAUSE / PLAY
                            (
                                Button,
                                ButtonConfig { hover: false },
                                TimeButton { id: 0 },
                                GUIButton::Pause,
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    border: UiRect::all(px(2)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(Text::new("||"),),],
                            ),
                            // SPEED 1
                            (
                                Button,
                                ButtonConfig { hover: false },
                                TimeButton { id: 1 },
                                GUIButton::Speed(1),
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    border: UiRect::all(px(2)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(Text::new("1"),),],
                            ),
                            // SPEED 2
                            (
                                Button,
                                ButtonConfig { hover: false },
                                TimeButton { id: 2 },
                                GUIButton::Speed(2),
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    border: UiRect::all(px(2)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(Text::new("2"),),],
                            ),
                            // SPEED 3
                            (
                                Button,
                                ButtonConfig { hover: false },
                                TimeButton { id: 3 },
                                GUIButton::Speed(3),
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    border: UiRect::all(px(2)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(Text::new("3"),),],
                            ),
                            // SPEED 4
                            (
                                Button,
                                ButtonConfig { hover: false },
                                TimeButton { id: 4 },
                                GUIButton::Speed(4),
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    border: UiRect::all(px(2)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(Text::new("4"),),],
                            ),
                            // SPEED 5
                            (
                                Button,
                                ButtonConfig { hover: false },
                                TimeButton { id: 5 },
                                GUIButton::Speed(5),
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    border: UiRect::all(px(2)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(Text::new("5"),),],
                            ),
                        ],
                    ),
                ],
            ),
            // ============================================================
            // MIDDLE AREA
            // ============================================================
            (
                Node {
                    width: percent(100),
                    height: percent(100),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    // ====================================================
                    // LEFT MENU
                    // ====================================================
                    (
                        Node {
                            width: px(60),
                            height: percent(100),
                            top: px(50),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::top(px(10)),
                            row_gap: px(8),
                            ..default()
                        },
                        BorderColor {
                            right: Color::WHITE,
                            ..default()
                        },
                        children![
                            // POLITICS
                            (
                                Button,
                                GlobalZIndex(10),
                                ButtonConfig { hover: true },
                                GUIButton::Politics,
                                Node {
                                    width: px(44),
                                    height: px(44),
                                    border: UiRect::all(px(2)),
                                    border_radius: BorderRadius::MAX,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(
                                    ImageNode::new(asset_server.load("gfx/menu/politics.png")),
                                    Node {
                                        width: percent(85),
                                        height: percent(85),
                                        ..default()
                                    }
                                ),],
                            ),
                            // PRODUCTION
                            (
                                Button,
                                GlobalZIndex(10),
                                ButtonConfig { hover: true },
                                GUIButton::Production,
                                Node {
                                    width: px(44),
                                    height: px(44),
                                    border: UiRect::all(px(2)),
                                    border_radius: BorderRadius::MAX,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(
                                    ImageNode::new(asset_server.load("gfx/menu/production.png")),
                                    Node {
                                        width: percent(85),
                                        height: percent(85),
                                        ..default()
                                    }
                                ),],
                            ),
                            // TREASURY
                            (
                                Button,
                                GlobalZIndex(10),
                                ButtonConfig { hover: true },
                                GUIButton::Treasury,
                                Node {
                                    width: px(44),
                                    height: px(44),
                                    border: UiRect::all(px(2)),
                                    border_radius: BorderRadius::MAX,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(
                                    ImageNode::new(asset_server.load("gfx/menu/budget.png")),
                                    Node {
                                        width: percent(85),
                                        height: percent(85),
                                        ..default()
                                    }
                                ),],
                            ),
                            // MILITARY
                            (
                                Button,
                                GlobalZIndex(10),
                                ButtonConfig { hover: true },
                                GUIButton::Military,
                                Node {
                                    width: px(44),
                                    height: px(44),
                                    border: UiRect::all(px(2)),
                                    border_radius: BorderRadius::MAX,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(
                                    ImageNode::new(asset_server.load("gfx/menu/military.png")),
                                    Node {
                                        width: percent(85),
                                        height: percent(85),
                                        ..default()
                                    }
                                ),],
                            ),
                            // RESEARCH
                            (
                                Button,
                                GlobalZIndex(10),
                                ButtonConfig { hover: true },
                                GUIButton::Research,
                                Node {
                                    width: px(44),
                                    height: px(44),
                                    border: UiRect::all(px(2)),
                                    border_radius: BorderRadius::MAX,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor::all(Color::WHITE),
                                BackgroundColor(Color::BLACK),
                                children![(
                                    ImageNode::new(asset_server.load("gfx/menu/techtree.png")),
                                    Node {
                                        width: percent(85),
                                        height: percent(85),
                                        ..default()
                                    }
                                ),],
                            ),
                        ],
                    ),
                    // ====================================================
                    // GAME AREA
                    // ====================================================
                    (Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },),
                    // ====================================================
                    // RIGHT PANEL
                    // ====================================================
                    (
                        Node {
                            width: px(240),
                            height: percent(100),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(px(10)),
                            row_gap: px(10),
                            ..default()
                        },
                        BorderColor {
                            left: Color::WHITE,
                            ..default()
                        },
                        children![
                            ((
                                Node {
                                    width: px(240),
                                    height: px(210),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0., 0., 0., 0.5)),
                                BorderColor::all(Color::WHITE),
                                children![
                                    // =====================================================
                                    // Header
                                    // =====================================================
                                    (
                                        Node {
                                            width: percent(100),
                                            height: px(35),
                                            flex_shrink: 0.0,
                                            padding: UiRect::horizontal(px(8)),
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.18, 0.18, 0.18,)),
                                        children![(Text::new("Notifications"),),],
                                    ),
                                    // =====================================================
                                    // Scroll-Bereich
                                    // =====================================================
                                    (
                                        ScrollArea,
                                        Node {
                                            width: percent(100),
                                            height: px(175),
                                            flex_shrink: 0.0,
                                            overflow: Overflow::scroll_y(),
                                            ..default()
                                        },
                                        ScrollPosition::default(),
                                        children![
                                            // =================================================
                                            // Inhalt der ScrollArea
                                            // =================================================
                                            (
                                                NotificationList,
                                                Node {
                                                    width: percent(100),
                                                    flex_direction: FlexDirection::Column,
                                                    align_items: AlignItems::Stretch,
                                                    row_gap: px(4),
                                                    padding: UiRect::all(px(5)),
                                                    ..default()
                                                },
                                                children![],
                                            ),
                                        ],
                                    ),
                                ],
                            ))
                        ],
                    ),
                ],
            ),
        ],
    ));
}

fn gui_button_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &GUIButton), Changed<Interaction>>,
    mut client_messages: MessageWriter<ClientMessage>,
    mut notification_query: Query<&mut NotificationItem>,
    submenu_query: Query<Entity, With<SubMenu>>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button {
            GUIButton::Politics => {
                open_submenu(&mut commands, &submenu_query, "Politics");
            }

            GUIButton::Production => {
                open_submenu(&mut commands, &submenu_query, "Production");
            }

            GUIButton::NationView => {
                open_submenu(&mut commands, &submenu_query, "Nation");
            }

            GUIButton::Treasury => {
                open_submenu(&mut commands, &submenu_query, "Treasury");
            }

            GUIButton::Military => {
                open_submenu(&mut commands, &submenu_query, "Military");
            }

            GUIButton::Research => {
                open_submenu(&mut commands, &submenu_query, "Research");
            }

            GUIButton::Pause => {
                client_messages.write(ClientMessage {
                    msg: ("time change".to_string()),
                });
            }

            GUIButton::Speed(speed) => {
                client_messages.write(ClientMessage {
                    msg: (format!("time speed {}", speed)),
                });
            }

            GUIButton::Notification(not_entity) => {
                let Ok(mut notification) = notification_query.get_mut(*not_entity) else {
                    continue;
                };

                if (notification.open) {
                    continue;
                }

                let (_window, content) = crate::render::window::spawn_notification_window(
                    &mut commands,
                    &notification.title,
                    400.0,
                    200.0,
                    500.0,
                    300.0,
                    *not_entity,
                );

                commands.entity(content).with_children(|parent| {
                    parent.spawn((Text::new(&notification.content),));
                });

                notification.open = true;
            }
        }
    }
}

fn cleanup_gui(mut commands: Commands, entities: Query<Entity, With<GUIEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn open_submenu(
    commands: &mut Commands,
    submenu_query: &Query<Entity, With<SubMenu>>,
    title: &str,
) -> Entity {
    // Altes Menü löschen
    for entity in submenu_query.iter() {
        commands.entity(entity).despawn();
    }

    // Neues Menü erstellen
    commands
        .spawn((
            SubMenu,
            GlobalZIndex(5),
            Node {
                position_type: PositionType::Absolute,
                left: px(0.),
                top: px(80.0),
                width: px(400.0),
                height: percent(100.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
            BorderColor::all(Color::WHITE),
            children![(
                Node {
                    width: percent(100.0),
                    height: px(40.0),
                    padding: UiRect::horizontal(px(10.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                children![
                    (Text::new(title),),
                    (
                        Button,
                        ButtonConfig { hover: true },
                        SubMenuCloseButton,
                        Node {
                            height: percent(100),
                            aspect_ratio: Some(1.),
                            right: px(0.),
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::BLACK),
                        BorderColor::all(Color::WHITE),
                        children![(Text::new("X"),)],
                    ),
                ],
            ),],
        ))
        .id()
}

fn submenu_close_system(
    mut commands: Commands,
    query: Query<&Interaction, (Changed<Interaction>, With<SubMenuCloseButton>)>,
    submenu_query: Query<Entity, With<SubMenu>>,
) {
    for interaction in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(submenu) = submenu_query.single() {
            commands.entity(submenu).despawn();
        }
    }
}