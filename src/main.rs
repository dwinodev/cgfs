// use rust_computer_graphics_from_scratch::canvas::{Canvas, Rgb};
mod canvas;
use crate::canvas::*;
const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const BACKGROUND_COLOR: (i16, i16, i16) = (255, 255, 255);
const VIEWPORT_WIDTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 1.0;
const PROJECTION_PLANE_D: f64 = 1.0;
const ANIMATED: bool = false;

fn main() {
    ////////
    // SCENE
    ////////$

    let mut scene = Scene::new();

    scene.spheres.push(Sphere::new(
        Vec3::new(0.0, -1.0, 3.0),
        1.0,
        Rgb::from_ints(255, 0, 0),
        500,
    ));

    scene.spheres.push(Sphere::new(
        Vec3::new(2.0, 0.0, 4.0),
        1.0,
        Rgb::from_ints(0, 0, 255),
        500,
    ));

    scene.spheres.push(Sphere::new(
        Vec3::new(-2.0, 0.0, 4.0),
        1.0,
        Rgb::from_ints(0, 255, 0),
        10,
    ));

    scene.spheres.push(Sphere::new(
        Vec3::new(0.0, -5001.0, 0.0),
        5000.0,
        Rgb::from_ints(255, 255, 0),
        1000,
    ));

    scene.lights.push(Light::new(
        Vec3::new(0.0, 0.0, 0.0),
        LightType::AMBIENT,
        0.2,
        Vec3::new(0.0, 0.0, 0.0),
    ));

    scene.lights.push(Light::new(
        Vec3::new(2.0, 1.0, 0.0),
        LightType::POINT,
        0.6,
        Vec3::new(0.0, 0.0, 0.0),
    ));

    scene.lights.push(Light::new(
        Vec3::new(0.0, 0.0, 0.0),
        LightType::DIRECTIONAL,
        0.2,
        Vec3::new(1.0, 4.0, 4.0),
    ));

    let mut my_canvas = Canvas::new("anouk", WIDTH, HEIGHT);

    let o = Vec3::new(0.0, 0.0, 0.0);

    let cw = WIDTH as i32;
    let ch = HEIGHT as i32;

    if ANIMATED {
        while my_canvas.window.is_open() && !my_canvas.window.is_key_down(minifb::Key::Escape) {
            // scene.spheres[0].center.y -= 0.001;
            // scene.spheres[1].center.y += 0.01;
            // scene.spheres[2].center.y += 0.02;
            // scene.spheres[3].center.y += 0.03;
            scene.lights[1].position.y += 0.1;
            scene.lights[1].position.x += 0.1;

            for x in -cw / 2..cw / 2 {
                for y in -ch / 2..ch / 2 {
                    let d = canvas_to_viewport(x, y);
                    let color = trace_ray(o, d, 1.0, f64::INFINITY, &scene);
                    my_canvas.put_pixel(x, y, &color);
                }
            }

            //println!("{}", scene.spheres[1].center.y);
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            my_canvas
                .window
                .update_with_buffer(&my_canvas.buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    } else {
        for x in -cw / 2..cw / 2 {
            for y in -ch / 2..ch / 2 {
                let d = canvas_to_viewport(x, y);
                let color = trace_ray(o, d, 1.0, f64::INFINITY, &scene);
                my_canvas.put_pixel(x, y, &color);
            }
        }
        my_canvas.display_until_exit();
    }
}

fn canvas_to_viewport(x: i32, y: i32) -> Vec3 {
    Vec3::new(
        x as f64 * (VIEWPORT_WIDTH / WIDTH as f64),
        y as f64 * (VIEWPORT_HEIGHT / HEIGHT as f64),
        PROJECTION_PLANE_D,
    )
}

fn trace_ray(o: Vec3, d: Vec3, t_min: f64, t_max: f64, scene: &Scene) -> Rgb {
    // let mut closest_t = t_max;
    // let mut closest_sphere = None;

    let (closest_sphere, closest_t) = closest_intersection(o, d, t_min, t_max, scene);
    // for sphere in &scene.spheres {
    //     let (t1, t2) = intersect_ray_sphere(o, d, &sphere);
    //     if t1 > t_min && t1 < t_max && t1 < closest_t {
    //         closest_t = t1;
    //         closest_sphere = Some(sphere);
    //     }
    //     if t2 > t_min && t2 < t_max && t2 < closest_t {
    //         closest_t = t2;
    //         closest_sphere = Some(sphere);
    //     }
    // }
    match closest_sphere {
        None => Rgb::from_ints(BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2),
        Some(s) => {
            let p = o.add(d.scale(closest_t));
            let n = p.substract(s.center);
            let n = n.normalize();

            s.color
                .multiply_by(compute_lighting(p, n, d.scale(-1.0), s.specular, scene))
                .clamp()
        }
    }
}

fn intersect_ray_sphere(o: Vec3, d: Vec3, sphere: &Sphere) -> (f64, f64) {
    let r = sphere.radius;
    let co = o.substract(sphere.center);

    let a = d.dot(d);
    let b = 2.0 * co.dot(d);
    let c = co.dot(co) - (r * r);

    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant < 0.0 {
        (f64::INFINITY, f64::INFINITY)
    } else {
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        (t1, t2)
    }
}

fn closest_intersection(
    o: Vec3,
    d: Vec3,
    t_min: f64,
    t_max: f64,
    scene: &Scene,
) -> (Option<&Sphere>, f64) {
    let mut closest_t = t_max;
    let mut closest_sphere = None;
    for sphere in &scene.spheres {
        let (t1, t2) = intersect_ray_sphere(o, d, &sphere);
        if t1 > t_min && t1 < t_max && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if t2 > t_min && t2 < t_max && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }

    (closest_sphere, closest_t)
}

fn compute_lighting(p: Vec3, n: Vec3, v: Vec3, s: i32, scene: &Scene) -> f64 {
    let mut i = 0.0;
    let mut t_max = 0.0;
    for light in &scene.lights {
        if light.light_type == LightType::AMBIENT {
            i += light.intensity
        } else {
            let mut l = Vec3::new(0.0, 0.0, 0.0);
            if light.light_type == LightType::POINT {
                l = light.position.substract(p);
                t_max = 1.0;
            } else {
                l = light.direction;
                t_max = f64::INFINITY;
            }

            //Shadow Check
            let shadow_sphere = closest_intersection(p, l, 0.001, t_max, scene).0;

            if let None = shadow_sphere {
                //Diffuse
                let n_dot_l = n.dot(l);
                if n_dot_l > 0.0 {
                    i += light.intensity * n_dot_l / (n.length() * l.length());
                }

                //Specular
                if s != -1 {
                    let r = n.scale(2.0).scale(n.dot(l)).substract(l);
                    let r_dot_v = r.dot(v);
                    if r_dot_v > 0.0 {
                        i += light.intensity * (r_dot_v / (r.length() * v.length())).powi(s);
                    }
                }
            }
        }
    }
    i
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    fn add(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    fn substract(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
    fn scale(&self, scale: f64) -> Vec3 {
        Vec3::new(self.x * scale, self.y * scale, self.z * scale)
    }
    fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn normalize(&self) -> Vec3 {
        let l = self.length();
        Vec3::new(self.x / l, self.y / l, self.z / l)
    }
}

struct Scene {
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
}

impl Scene {
    fn new() -> Self {
        Self {
            spheres: Vec::new(),
            lights: Vec::new(),
        }
    }
}

struct Sphere {
    center: Vec3,
    radius: f64,
    color: Rgb,
    specular: i32,
}
impl Sphere {
    fn new(center: Vec3, radius: f64, color: Rgb, specular: i32) -> Self {
        Self {
            center,
            radius,
            color,
            specular,
        }
    }
}

struct Light {
    position: Vec3,
    light_type: LightType,
    intensity: f64,
    direction: Vec3,
}

impl Light {
    fn new(position: Vec3, light_type: LightType, intensity: f64, direction: Vec3) -> Self {
        Self {
            position,
            light_type,
            intensity,
            direction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum LightType {
    AMBIENT,
    DIRECTIONAL,
    POINT,
}
