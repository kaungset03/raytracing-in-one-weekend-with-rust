use ray::Ray;
use vec3::{dot, unit_vector, Color, Point3, Vec3};

pub mod color;
pub mod ray;
pub mod vec3;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let b = -2.0 * dot(&oc, &r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Vectors across the viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width - 1) as f64;
    let pixel_delta_v = viewport_v / (image_height - 1) as f64;

    //let viewport_center = camera_center - Vec3::new(0.0, 0.0, focal_length);
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    // center of first pixel
    let pixel100_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in 0..image_height {
        eprintln!("\rScan lines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel100_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);
            color::write_color(pixel_color, &mut std::io::stdout()).unwrap();
        }
    }
    eprintln!("\nDone.");
}
