use server::core::map_gen;
fn main() {
    let map = map_gen::map_gen(20, 20, 42);
    println!("Generated map with {} provinces", map.provinces.len());
}