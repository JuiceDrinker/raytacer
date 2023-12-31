mod camera;
mod hit;
mod hit_record;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

use std::sync::Arc;

use hit::Hit;
use rand::Rng;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;

use crate::camera::Camera;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::world::World;

type Color = Vec3;
type Point3 = Vec3;

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

fn ray_color(ray: &Ray, world: &World, depth: u64) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit_record) = world.hit(ray, 0.001, std::f64::INFINITY) {
        if let Some((a, s)) = hit_record.material.scatter(ray, &hit_record) {
            a * ray_color(&s, world, depth - 1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 1500;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 20;
    const MAX_DEPTH: u64 = 20;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(0.0, 1.5, -3.0);
    let lookat = Point3::new(0.0, 1.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        70.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);

        let scanlines: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map_init(rand::thread_rng, |rng, i| {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();
                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let ray = cam.get_ray(u, v);
                    pixel_color = pixel_color + ray_color(&ray, &world, MAX_DEPTH)
                }

                pixel_color
            })
            .collect();

        for line in scanlines {
            println!("{}", line.format_color(SAMPLES_PER_PIXEL));
        }
    }

    log::debug!("\rDone!");
}
