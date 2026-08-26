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

pub fn delete_display(
    commands: &mut Commands,
    displays: Query<(Entity, &TextSystem), With<TextSystem>>,
    id: String,
) {
    for (entity, text_system) in &displays {
        if text_system.id == id {
            commands.entity(entity).despawn();
        }
    }
}