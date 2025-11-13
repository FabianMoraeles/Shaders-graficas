mod geom;
mod body;
mod shaders;

use raylib::prelude::*;
use geom::{vec3, Vec3};
use body::{BodyShader, CelestialBody};

struct Ring {
    center: Vec3,
    normal: Vec3,
    inner_radius: f32,
    outer_radius: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(900, 520)
        .title("Sistema Solar")
        .build();

    rl.set_target_fps(60);

    let mut sun = CelestialBody::new(
        vec3(0.0, 0.0, 6.0),
        1.61,
        BodyShader::Sun,
    );

    let mut mercury = CelestialBody::new(
        vec3(2.2, 0.0, 6.0),
        0.4,
        BodyShader::Mercury,
    );

    let mut earth = CelestialBody::new(
        vec3(3.2, 0.0, 6.0),
        0.7,
        BodyShader::Earth,
    );

    let mut mars = CelestialBody::new(
        vec3(4.4, 0.0, 6.0),
        0.5,
        BodyShader::Mars,
    );

    let mut saturn = CelestialBody::new(
        vec3(6.0, 0.0, 6.0),
        0.9,
        BodyShader::Saturn,
    );

    let sun_pos = sun.center;

    let mercury_orbit_radius = (mercury.center - sun_pos).norm();
    let earth_orbit_radius   = (earth.center   - sun_pos).norm();
    let mars_orbit_radius    = (mars.center    - sun_pos).norm();
    let saturn_orbit_radius  = (saturn.center  - sun_pos).norm();

    let mercury_base_y = mercury.center.y;
    let earth_base_y   = earth.center.y;
    let mars_base_y    = mars.center.y;
    let saturn_base_y  = saturn.center.y;

    let mercury_orbit_speed = 0.9;
    let earth_orbit_speed   = 0.4;
    let mars_orbit_speed    = 0.25;
    let saturn_orbit_speed  = 0.18;

    let mut saturn_rings = Ring {
        center: saturn.center,
        normal: vec3(0.0, 1.0, 0.3).normalize(),
        inner_radius: saturn.radius * 1.8,
        outer_radius: saturn.radius * 3.0,
    };

    while !rl.window_should_close() {
        let time = rl.get_time() as f32;

        let sun_pos = sun.center;

        let mercury_angle = time * mercury_orbit_speed;
        mercury.center = vec3(
            sun_pos.x + mercury_orbit_radius * mercury_angle.cos(),
            mercury_base_y,
            sun_pos.z + mercury_orbit_radius * mercury_angle.sin(),
        );

        let earth_angle = time * earth_orbit_speed;
        earth.center = vec3(
            sun_pos.x + earth_orbit_radius * earth_angle.cos(),
            earth_base_y,
            sun_pos.z + earth_orbit_radius * earth_angle.sin(),
        );

        let mars_angle = time * mars_orbit_speed;
        mars.center = vec3(
            sun_pos.x + mars_orbit_radius * mars_angle.cos(),
            mars_base_y,
            sun_pos.z + mars_orbit_radius * mars_angle.sin(),
        );

        let saturn_angle = time * saturn_orbit_speed;
        saturn.center = vec3(
            sun_pos.x + saturn_orbit_radius * saturn_angle.cos(),
            saturn_base_y,
            sun_pos.z + saturn_orbit_radius * saturn_angle.sin(),
        );

        saturn_rings.center = saturn.center;

        let bodies: [&CelestialBody; 5] = [&sun, &mercury, &earth, &mars, &saturn];

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let screen_w = d.get_screen_width();
        let screen_h = d.get_screen_height();

        let camera_pos = vec3(0.0, 0.0, 0.0);

        for y in 0..screen_h {
            for x in 0..screen_w {
                let nx = (x as f32 / screen_w as f32) * 2.0 - 1.0;
                let ny = 1.0 - (y as f32 / screen_h as f32) * 2.0;

                let ray_dir = vec3(nx * 1.2, ny, 1.5).normalize();

                let mut closest_t = f32::INFINITY;
                let mut pixel_color = Color::BLACK;

                for body_ref in bodies.iter() {
                    let body = *body_ref;

                    if let Some((t, normal)) =
                        intersect_sphere(camera_pos, ray_dir, body.center, body.radius)
                    {
                        if t < closest_t {
                            closest_t = t;
                            let hit_pos = camera_pos + ray_dir * t;

                            let light_dir = match body.shader {
                                BodyShader::Sun => (hit_pos - sun_pos).normalize(),
                                _ => (sun_pos - hit_pos).normalize(),
                            };

                            let c = shaders::shade_body(
                                body.shader,
                                hit_pos,
                                normal,
                                light_dir,
                                time,
                            );
                            pixel_color = c;
                        }
                    }
                }

                if let Some((t_ring, normal_ring)) =
                    intersect_ring(camera_pos, ray_dir, &saturn_rings)
                {
                    if t_ring < closest_t {
                        closest_t = t_ring;
                        let hit_pos = camera_pos + ray_dir * t_ring;

                        let light_dir = (sun_pos - hit_pos).normalize();
                        let c = shaders::shade_saturn_rings(
                            hit_pos,
                            normal_ring,
                            light_dir,
                            time,
                            saturn_rings.center,
                            saturn_rings.inner_radius,
                            saturn_rings.outer_radius,
                        );
                        pixel_color = c;
                    }
                }

                d.draw_pixel(x, y, pixel_color);
            }
        }
    }
}

fn intersect_sphere(
    ray_origin: Vec3,
    ray_dir: Vec3,
    center: Vec3,
    radius: f32,
) -> Option<(f32, Vec3)> {
    let oc = ray_origin - center;
    let a: f32 = ray_dir.dot(&ray_dir);
    let b: f32 = 2.0 * oc.dot(&ray_dir);
    let c: f32 = oc.dot(&oc) - radius * radius;

    let disc: f32 = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }

    let sqrt_disc = disc.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);

    let t = if t1 > 0.0 {
        t1
    } else if t2 > 0.0 {
        t2
    } else {
        return None;
    };

    let hit = ray_origin + ray_dir * t;
    let normal = (hit - center).normalize();

    Some((t, normal))
}

fn intersect_ring(
    ray_origin: Vec3,
    ray_dir: Vec3,
    ring: &Ring,
) -> Option<(f32, Vec3)> {
    let n = ring.normal.normalize();
    let denom = n.dot(&ray_dir);
    if denom.abs() < 1e-4 {
        return None;
    }

    let t = n.dot(&(ring.center - ray_origin)) / denom;
    if t <= 0.0 {
        return None;
    }

    let hit_pos = ray_origin + ray_dir * t;

    let v = hit_pos - ring.center;
    let v_proj = v - n * v.dot(&n);
    let r = v_proj.norm();

    if r < ring.inner_radius || r > ring.outer_radius {
        return None;
    }

    let mut normal = n;
    if normal.dot(&ray_dir) > 0.0 {
        normal = -normal;
    }

    Some((t, normal))
}
