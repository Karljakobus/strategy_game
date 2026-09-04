use bevy::prelude::*;
use bevy::window::{CursorIcon, SystemCursorIcon};

#[derive(Component)]
pub struct ButtonConfig {
    pub hover: bool,
}

pub fn button_cursor_system(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    mut query: Query<(
        Entity,
        &Interaction,
        &ButtonConfig,
        &mut BackgroundColor,
        &mut BorderColor,
    ), 
    (
        Changed<Interaction>, 
        With<Button>
    )
    >,
) {
    for (entity, interaction, config, mut back_color, mut border_color) in &mut query {
        match *interaction {
            Interaction::Hovered => {
                commands.entity(*window)
                    .insert(CursorIcon::from(SystemCursorIcon::Pointer));
                if (config.hover) {
                    *back_color = Color::srgb(0.25, 0.25, 0.25).into();
                }
            }

            Interaction::None => {
                commands.entity(*window)
                    .insert(CursorIcon::from(SystemCursorIcon::Default));
                if (config.hover) {
                    *back_color = Color::BLACK.into();
                }
            }

            Interaction::Pressed => {
                commands.entity(*window)
                    .insert(CursorIcon::from(SystemCursorIcon::Default));
                if (config.hover) {
                    *back_color = Color::srgb(0.35, 0.35, 0.35).into();
                }
            }
        }
    }
}