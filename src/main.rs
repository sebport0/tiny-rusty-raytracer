use std::fs::File;
use std::io::prelude::*;
use tiny_rusty_raytracer::{cast_ray, Material, Sphere, Vector3};

fn main() {
    let ivory = Material::new(Vector3::new(0.4, 0.4, 0.3));
    let red_rubber = Material::new(Vector3::new(0.3, 0.1, 0.1));
    let spheres = vec![
        Sphere::new(Vector3::new(-3.0, 0.0, -16.0), 2.0, ivory),
        Sphere::new(Vector3::new(1.5, -0.5, -18.0), 3.0, red_rubber),
        Sphere::new(Vector3::new(-1.0, -1.5, -12.0), 2.0, red_rubber),
        Sphere::new(Vector3::new(7.0, 5.0, -18.0), 4.0, ivory),
    ];

    render(&spheres);
}

fn render(spheres: &Vec<Sphere>) {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    const PI: f64 = std::f64::consts::PI;
    const FOV: f64 = PI / 2.0;

    let origin: Vector3 = Vector3::new_zero();
    let mut framebuffer: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    // TODO: parallelize the for below.
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x: f64 = (2.0 * (i as f64 + 0.5) / WIDTH as f64 - 1.0)
                * (FOV / 2.0).tan()
                * (WIDTH as f64 / HEIGHT as f64);
            let y: f64 = -(2.0 * j as f64 / HEIGHT as f64 - 1.0) * (FOV / 2.0).tan();
            let dir = Vector3::new(x, y, -1.0).normalize();
            let index = i + j * WIDTH;
            framebuffer[index] = cast_ray(&origin, &dir, spheres);
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
