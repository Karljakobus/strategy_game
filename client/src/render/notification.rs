use bevy::prelude::*;

use crate::render::{button_systems::ButtonConfig, game_gui::{GUIButton, NotificationItem}};

pub fn add_notification(
    commands: &mut Commands,
    notification_list: Entity,
    title: String,
    content: String,
) {
    let notification = commands
        .spawn((
            Button,
            ButtonConfig { hover: true },
            NotificationItem {
                title,
                content,
                open: false,
            },

            Node {
                width: percent(100),
                height: px(35),
                flex_shrink: 0.0,
                padding: UiRect::horizontal(px(5)),
                align_items: AlignItems::Center,
                ..default()
            },

            BackgroundColor(Color::BLACK),

            children![
                (
                    Text::new("Notification"),
                ),
            ],
        ))
        .id();

    commands
        .entity(notification)
        .insert(GUIButton::Notification(notification));

    commands
        .entity(notification_list)
        .add_child(notification);
}
