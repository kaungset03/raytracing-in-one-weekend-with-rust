use color::{write_color, Color};
use ray::Ray;
use vec3::{dot, Point3, Vec3};

pub mod color;
pub mod ray;
pub mod vec3;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = center - *r.origin();
    let a = dot(*r.direction(), *r.direction());
    let b = -2.0 * dot(*r.direction(), oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.direction();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width = 400;

    // calculate image height
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // calculate the horizontal and vertical vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // calculate the horizontal and vertical deltas
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // calculate the location of upper left pixel
    let viewport_upper_left =
        camera_center - (viewport_u / 2.0) - (viewport_v / 2.0) - Vec3::new(0.0, 0.0, focal_length);
    let pixel100_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScan lines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel100_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);
            write_color(pixel_color, |s| println!("{}", s));
        }
    }

    eprintln!("\nDone.");
}
