use bevy::{prelude::*,};

pub fn create_polygon_mesh(vertices: Vec<Vec2>) -> Mesh {
    let mut indices = Vec::new();

    for i in 1..vertices.len() - 1 {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }

    let positions: Vec<[f32; 3]> = vertices
        .iter()
        .map(|v| [v.x, v.y, 0.0])
        .collect();

    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    )

    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}

pub fn create_outline_mesh(points: &[Vec2], width: f32) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let half_width = width / 2.0;

    for i in 0..points.len() {
        let a = points[i];

        // Beim letzten Punkt wieder zum ersten Punkt
        let b = points[(i + 1) % points.len()];

        // Richtung der Kante
        let direction = (b - a).normalize();

        // Senkrechter Vektor
        let normal = Vec2::new(-direction.y, direction.x);

        // Die vier Ecken des Rechtecks
        let v0 = a + normal * half_width;
        let v1 = a - normal * half_width;
        let v2 = b - normal * half_width;
        let v3 = b + normal * half_width;

        let start = vertices.len() as u32;

        vertices.push([v0.x, v0.y, 0.0]);
        vertices.push([v1.x, v1.y, 0.0]);
        vertices.push([v2.x, v2.y, 0.0]);
        vertices.push([v3.x, v3.y, 0.0]);

        // Zwei Dreiecke
        indices.extend_from_slice(&[
            start,
            start + 1,
            start + 2,

            start,
            start + 2,
            start + 3,
        ]);
    }

    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(
        bevy::mesh::Indices::U32(indices),
    )
}