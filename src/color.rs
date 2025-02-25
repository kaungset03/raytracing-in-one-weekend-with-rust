use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<F>(pixel_color: Color, mut print_fn: F)
where
    F: FnMut(String),
{
    let ir = (255.999 * pixel_color.x) as i32;
    let ig = (255.999 * pixel_color.y) as i32;
    let ib = (255.999 * pixel_color.z) as i32;

    print_fn(format!("{} {} {}\n", ir, ig, ib));
}
