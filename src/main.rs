use std::fs::File;
use std::io::prelude::*;
use tiny_rusty_raytracer::Vector3;

fn main() {
    render();
}

fn render() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    let mut framebuffer: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let index = i + j * WIDTH;
            framebuffer[index] =
                Vector3::new(j as f64 / HEIGHT as f64, i as f64 / WIDTH as f64, 0.0);
        }
    }

    let mut ofs = File::create("out.ppm").unwrap();
    write!(ofs, "P6\n{} {}\n255\n", WIDTH, HEIGHT).unwrap();
}
