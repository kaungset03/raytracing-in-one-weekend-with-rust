use color::{write_color, Color};
use ray::Ray;
use vec3::{dot, Point3, Vec3};

pub mod color;
pub mod ray;
pub mod vec3;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - *r.origin();
    let a = r.direction().length_squared();
    let h = dot(*r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: Ray) -> Color {
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&sphere_center, 0.5, &r);
    if t > 0.0 {
        // normal vector to the sphere
        let n = r.at(t) - sphere_center;
        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
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
    let viewport_center = camera_center - Vec3::new(0.0, 0.0, focal_length);
    let viewport_upper_left = viewport_center - (viewport_u + viewport_v) * 0.5;
    let pixel100_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScan lines remaining: {} ", image_height - j);
        for i in 0..image_width {
            // move from left to right, top to bottom
            let pixel_center =
                pixel100_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            // ray from camera to pixel
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);
            write_color(pixel_color, |s| println!("{}", s));
        }
    }

    eprintln!("\nDone.");
}
