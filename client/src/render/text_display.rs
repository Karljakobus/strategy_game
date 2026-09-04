use bevy::{prelude::*,};

#[derive(Component)]
pub struct TextSystem {
    pub id: String
}

pub fn create_text(
    commands: &mut Commands,
    text: String,
    id: String,
    x: i32,
    y: i32,
) {
    commands.spawn((
        Text::new(text),
        GlobalZIndex(20),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(y),
            left: px(x),
            ..default()
        },
        TextSystem {
            id
        },
    ));
}

pub fn create_text_ur(
    commands: &mut Commands,
    text: String,
    id: String,
    x: i32,
    y: i32,
) {
    commands.spawn((
        Text::new(text),
        Node {
            position_type: PositionType::Absolute,
            top: px(y),
            right: px(x),
            ..default()
        },
        TextSystem {
            id
        },
    ));
}

pub fn create_text_ul(
    commands: &mut Commands,
    text: String,
    id: String,
    x: i32,
    y: i32,
) {
    commands.spawn((
        Text::new(text),
        Node {
            position_type: PositionType::Absolute,
            top: px(y),
            left: px(x),
            ..default()
        },
        TextSystem {
            id
        },
    ));
}

pub fn create_text_dr(
    commands: &mut Commands,
    text: String,
    id: String,
    x: i32,
    y: i32,
) {
    commands.spawn((
        Text::new(text),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(y),
            right: px(x),
            ..default()
        },
        TextSystem {
            id
        },
    ));
}

pub fn modify_display(
    text: String,
    displays: &mut Query<(&TextSystem, &mut Text)>,
    id: String,
) {
    for (text_system, mut text_component) in displays {
        if text_system.id == id {
            *text_component = Text::new(&text);
        }
    }
}

pub fn get_display_text(
    displays: Query<(&TextSystem, &Text)>,
    id: String,
) -> String {
    for (text_system, text) in &displays {
        if text_system.id == id {
            return text.0.clone();
        }
    }

    String::new()
}

pub fn delete_display(
    commands: &mut Commands,
    displays: Query<(Entity, &TextSystem),>,
    id: String,
) {
    for (entity, text_system) in &displays {
        if text_system.id == id {
            commands.entity(entity).despawn();
        }
    }
}