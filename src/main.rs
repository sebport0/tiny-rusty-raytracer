use std::fs::File;
use std::io::prelude::*;
use tiny_rusty_raytracer::Vector3;

fn main() {
    render();
}

fn render() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    let mut framebuffer: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.25); WIDTH * HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let index = i + j * WIDTH;
            framebuffer[index] =
                Vector3::new(j as f64 / HEIGHT as f64, i as f64 / WIDTH as f64, 0.0);
        }
    }

    let mut ofs = File::create("out.ppm").unwrap();
    write!(ofs, "P3\n{} {}\n255", WIDTH, HEIGHT).unwrap();

    for i in 0..(HEIGHT * WIDTH) {
        let red_value = (255.0 * framebuffer[i].x) as i32;
        let green_value = (255.0 * framebuffer[i].y) as i32;
        let blue_value = (255.0 * framebuffer[i].z) as i32;

        write!(ofs, "\n{} {} {}", red_value, green_value, blue_value).unwrap();
    }
}
