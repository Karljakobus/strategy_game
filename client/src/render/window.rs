use bevy::prelude::*;

use crate::render::button_systems::ButtonConfig;

#[derive(Component)]
pub struct GameWindow;

#[derive(Component)]
pub struct WindowDragBar;

#[derive(Component)]
pub struct WindowCloseButton {
    pub window: Entity,
    pub notification: Option<Entity>,
}

#[derive(Component)]
pub struct WindowContent;

#[derive(Resource, Default)]
pub struct DragState {
    pub window: Option<Entity>,
    pub offset: Vec2,
}

pub struct GameWindowPlugin;

impl Plugin for GameWindowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragState>();

        app.add_systems(
            Update,
            (
                window_drag_system,
                window_close_system,
            )
                .in_set(crate::GameSet),
        );
    }
}

pub fn spawn_window(
    commands: &mut Commands,
    title: &str,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    notification: Option<Entity>,
) -> (Entity, Entity) {
    let window = commands
        .spawn((
            GameWindow,
            GlobalZIndex(15),
            Node {
                position_type: PositionType::Absolute,
                left: px(left),
                top: px(top),
                width: px(width),
                height: px(height),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
            BorderColor::all(Color::WHITE),
        ))
        .id();

    // Dragbar
    let drag_bar = commands
        .spawn((
            Button,
            WindowDragBar,
            Node {
                width: percent(100),
                height: px(35),
                flex_shrink: 0.0,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::left(px(8)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
        ))
        .id();

    // Titel
    let title_text = commands
        .spawn((
            Text::new(title),
        ))
        .id();

    // Close Button
    let close_button = commands
        .spawn((
            Button,
            WindowCloseButton {
                window,
                notification,
            },
            ButtonConfig { hover: true },
            Node {
                width: px(30),
                height: px(30),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            BorderColor::all(Color::WHITE),
            children![
                (
                    Text::new("X"),
                ),
            ],
        ))
        .id();

    // Content
    let content = commands
        .spawn((
            WindowContent,
            Node {
                width: percent(100),
                height: percent(100),
                padding: UiRect::all(px(10)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .id();

    // Hierarchie
    commands.entity(window).add_child(drag_bar);
    commands.entity(window).add_child(content);

    commands.entity(drag_bar).add_child(title_text);
    commands.entity(drag_bar).add_child(close_button);

    (window, content)
}

pub fn spawn_notification_window(
    commands: &mut Commands,
    title: &str,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    notification: Entity,
) -> (Entity, Entity) {
    spawn_window(
        commands,
        title,
        x,
        y,
        width,
        height,
        Some(notification),
    )
}

// =============================================================
// Dragging
// =============================================================

fn window_drag_system(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DragState>,

    mut window_query: Query<&mut Node, With<GameWindow>>,

    drag_bar_query: Query<
        (&Interaction, &ChildOf),
        (With<WindowDragBar>, Changed<Interaction>),
    >,

    window: Query<&Window>,
) {
    // ---------------------------------------------------------
    // Drag starten
    // ---------------------------------------------------------

    if mouse_buttons.just_pressed(MouseButton::Left) {
        for (interaction, child_of) in &drag_bar_query {
            if *interaction != Interaction::Pressed {
                continue;
            }

            let window_entity = child_of.parent();

            let Ok(node) = window_query.get(window_entity) else {
                continue;
            };

            let Ok(window) = window.single() else {
                continue;
            };

            let Some(cursor) = window.cursor_position() else {
                continue;
            };

            let left = match node.left {
                Val::Px(value) => value,
                _ => 0.0,
            };

            let top = match node.top {
                Val::Px(value) => value,
                _ => 0.0,
            };

            drag_state.window = Some(window_entity);

            drag_state.offset =
                cursor - Vec2::new(left, top);

            break;
        }
    }

    // ---------------------------------------------------------
    // Fenster bewegen
    // ---------------------------------------------------------

    let Some(window_entity) = drag_state.window else {
        return;
    };

    if mouse_buttons.pressed(MouseButton::Left) {
        let Ok(mut node) = window_query.get_mut(window_entity) else {
            drag_state.window = None;
            return;
        };

        let Ok(window) = window.single() else {
            return;
        };

        let Some(cursor) = window.cursor_position() else {
            return;
        };

        let position = cursor - drag_state.offset;

        node.left = px(position.x);
        node.top = px(position.y);
    }

    // ---------------------------------------------------------
    // Drag beendet
    // ---------------------------------------------------------

    if mouse_buttons.just_released(MouseButton::Left) {
        drag_state.window = None;
    }
}

// =============================================================
// Close
// =============================================================

fn window_close_system(
    mut commands: Commands,
    query: Query<
        (&Interaction, &WindowCloseButton),
        Changed<Interaction>,
    >,
) {
    for (interaction, close_button) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Fenster schließen
        commands.entity(close_button.window).despawn();

        // Notification aus der Liste löschen
        if let Some(notification) = close_button.notification {
            commands.entity(notification).despawn();
        }
    }
}