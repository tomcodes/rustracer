mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    let oc: Vec3 = r.origin() - center;
    let a: f32 = Vec3::dot(&r.direction(), &r.direction());
    let b: f32 = 2.0 * Vec3::dot(&oc, &r.direction());
    let c: f32 = Vec3::dot(&oc, &oc) - (radius * radius);
    let discriminant: f32 = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn color(r: &Ray) -> Vec3 {
    let t: f32 = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, *r);

    if t > 0.0 {
        let N: Vec3 = Vec3::unit_vector(&(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)));

        return 0.5 * Vec3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }
    // if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, *r) {
    //     return Vec3::new(1.0, 0.0, 0.0)
    // }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let width: u32 = 200;
    let height: u32 = 100;
    let max_value: u32 = 255;

    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n{}", width, height, max_value);

    for j in (0..height).rev() {
        // eprintln!("Remaning lines: {}", j);

        for i in 0..width {
            let u: f32 = i as f32 / width as f32;
            let v: f32 = j as f32 / height as f32;

            let r: Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let col: Vec3 = color(&r);

            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    // eprintln!("All done!");
}
