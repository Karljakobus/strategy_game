
use rand::SeedableRng;
use rand::RngExt;
use std::collections::VecDeque;
use bevy::prelude::*;
use crate::engine::province_manager::{Province, TerrainType};

const CELL_SIZE: usize = 30;
pub struct MapData {
    pub width: usize,
    pub height: usize,
    pub provinces: Vec<Province>,
    pub rows : usize,
    pub cols : usize,
    pub province_pixel_map : Vec<usize>,
    pub border_pixel_map : Vec<bool>,
}

pub fn map_gen(cols: usize, rows: usize, seed: u64) -> MapData {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut map = generate_provinces(cols, rows, &mut rng);
    return map;
}

fn generate_provinces(cols: usize, rows: usize, rng: &mut rand::rngs::StdRng) -> MapData {
    let jitter: f32 = 0.7;
    let height : usize = rows * CELL_SIZE;
    let width : usize = cols * CELL_SIZE;
    let n = cols * rows;
    let mut size: Vec<usize> = vec![0; n];
    let mut sum_x: Vec<usize> = vec![0; n];
    let mut sum_y: Vec<usize> = vec![0; n];
    let jitter_px = (CELL_SIZE as f32 * jitter / 2.0) as i32;
    let half = (CELL_SIZE / 2) as i32;
    let mut queue : VecDeque<(usize, usize)> = VecDeque::new();
    let mut map = MapData {
        width : width,
        height : height,
        provinces:Vec::new(),
        rows : rows,
        cols : cols,
        province_pixel_map : vec![usize::MAX; width * height],
        border_pixel_map : vec![false; width * height],
    };
    for cy in 0..rows {
        for cx in 0..cols {
            let x : usize = cx * CELL_SIZE + (rng.random_range(-jitter_px..jitter_px) + half) as usize;
            let y : usize = cy * CELL_SIZE + (rng.random_range(-jitter_px..jitter_px) + half) as usize;
            let id : usize = cy * cols + cx;
            map.province_pixel_map[y * map.width + x] = id;
            size[id] += 1;
            sum_x[id] += x;
            sum_y[id] += y;
            queue.push_back((x, y));
        }
    } //erzeuge für jede Provinz einen Startpunkt mit einzigartiger id.
    loop{
        if queue.is_empty() {
            break;
        }
        let (x, y) = queue.pop_front().unwrap();
        let id = map.province_pixel_map[y * map.width + x];
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < map.width as i32 && ny >= 0 && ny < map.height as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if map.province_pixel_map[ny * map.width + nx] == usize::MAX {
                    map.province_pixel_map[ny * map.width + nx] = id;
                    size[id] += 1;
                    sum_x[id] += nx;
                    sum_y[id] += ny;
                    queue.push_back((nx, ny));
                }
            }
        }//flood fill damit die Provinzen ihre Fläche bekommen. Gleichzeitig wird die Fläche und der die summe der Koordinaten für die spätere Berechnung des Zentrums gespeichert.
    }
    for id in 0..n {
        map.provinces.push(Province {
            neighbors: Vec::new(),
            terrain: TerrainType::Ocean,
            size: size[id],
            center_pos: Vec2::new(
                sum_x[id] as f32 / size[id] as f32,
                sum_y[id] as f32 / size[id] as f32),
        })
    }//erstellt die Provinzen und berechnet das Zentrum jeder Provinz.
    for y in 0..map.height {
        for x in 0..map.width {
            let id = map.province_pixel_map[y * map.width + x];
            for (dx, dy) in [ (1, 0), (0, 1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < map.width as i32 && ny < map.height as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    let neighbor_id = map.province_pixel_map[ny * map.width + nx];
                    if neighbor_id != id {
                        if !map.provinces[id].neighbors.contains(&neighbor_id) {
                            map.provinces[id].neighbors.push(neighbor_id);
                            map.provinces[neighbor_id].neighbors.push(id);
                        }
                    map.border_pixel_map[y * map.width + x] = true;
                    }
                }
            }
        }
    }//erstellt die Nachbarschaftsbeziehungen zwischen den Provinzen. Und macht die boarder pixel map.
    return map;
}// Erstellt eine Karte mit Provinzen die alle Ozeane sind. Die Provinzen haben Nachbarschaftsbeziehungen und ein Zentrum.

fn generate_terrain(map: &mut MapData, rng: &mut rand::rngs::StdRng) {
     
}// generiert Kontinente und Terrain.
fn get_province_id(){
    
}