mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn color(r: &Ray) {
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
}

fn main() {
    let width: u32 = 200;
    let height: u32 = 100;
    // let max_value: u32 = 255;

    for j in (0..height).rev() {
        eprintln!("Remaning lines: {}", j);

        for i in 0..width {
            let r: f32 = i as f32 / width as f32;
            let g: f32 = j as f32 / height as f32;
            let b: f32 = 0.2;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("All done!");
}
