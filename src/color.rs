use crate::vec3::Color;

pub fn write_color(pixel_color: Color, out: &mut impl std::io::Write) -> std::io::Result<()> {
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;

    writeln!(out, "{} {} {}", r, g, b)
}
