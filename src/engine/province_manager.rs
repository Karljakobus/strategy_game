use bevy::{prelude::*,};
use crate::render::text_display;
use crate::render::text_display::*;

#[derive(Component)]
pub struct Province {
    pub points: Vec<Vec2>,
    pub id: String,
}

pub fn point_in_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    let mut inside = false;

    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[j];

        if ((a.y > point.y) != (b.y > point.y))
            && (point.x < (b.x - a.x) * (point.y - a.y)
                / (b.y - a.y)
                + a.x)
        {
            inside = !inside;
        }

        j = i;
    }

    inside
}

pub fn province_click_system(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    provinces: Query<(&Province, &GlobalTransform)>,
    mut commands: Commands,
    displays: Query<(Entity, &TextSystem),>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single().unwrap();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera.into_inner();

    let Ok(world_position) =
        camera.viewport_to_world_2d(camera_transform, cursor_position)
    else {
        return;
    };

    let mut found_province = false;

    for (province, transform) in &provinces {
        // Mausposition in lokale Polygonkoordinaten umwandeln
        let local_position = transform
            .affine()
            .inverse()
            .transform_point3(world_position.extend(0.0))
            .truncate();

        if point_in_polygon(local_position, &province.points) {
            println!("Province clicked!");
            text_display::delete_display(&mut commands, displays, "province".to_string());
            text_display::create_text(&mut commands, province.id.clone(), "province".to_string(), 12, 300);
            found_province = true;
        }
    }

    if !found_province {
        text_display::delete_display(&mut commands, displays, "province".to_string());
    }
}