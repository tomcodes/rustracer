mod vec3;

use vec3::Vec3;

fn write_ppm(w: u32, h: u32, max_value: u32) {
    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev() {
        eprintln!("Remaning lines: {}", j);

        for i in 0..w {
            let r: f32 = i as f32 / w as f32;
            let g: f32 = j as f32 / h as f32;
            let b: f32 = 0.2;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("All done!");
}

fn main() {
    let width: u32 = 256;
    let height: u32 = 256;
    let max_value: u32 = 255;

    write_ppm(width, height, max_value);

    let v: Vec3 = Vec3::new(2f32, 4f32, 6f32);
    let v2: Vec3 = Vec3::new(4f32, 12f32, 0.5f32);
    let v3: Vec3 = v + v2;

    println!("v3: {}", v3);
}
