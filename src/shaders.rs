use raylib::prelude::Color;

use crate::geom::Vec3;
use crate::body::BodyShader;

// ----------------- Helpers generales -----------------

fn clamp01(x: f32) -> f32 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b, a: 255 }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = clamp01(t);
    let r = lerp(a.r as f32, b.r as f32, t) as u8;
    let g = lerp(a.g as f32, b.g as f32, t) as u8;
    let b_ = lerp(a.b as f32, b.b as f32, t) as u8;
    Color { r, g, b: b_, a: 255 }
}

// Coordenadas "esféricas" aproximadas tomadas de la normal
fn spherical_coords(normal: Vec3) -> (f32, f32) {
    let n = normal.normalize();
    let lat = n.y;
    let lon = n.x.atan2(n.z);
    let lon_norm = (lon / std::f32::consts::PI).clamp(-1.0, 1.0);
    (lat, lon_norm)
}

// Iluminación básica
fn lambert(normal: Vec3, light_dir: Vec3) -> f32 {
    clamp01(normal.normalize().dot(&light_dir.normalize()))
}

// ----------------- API principal -----------------

pub fn shade_body(
    kind: BodyShader,
    pos: Vec3,
    normal: Vec3,
    light_dir: Vec3,
    time: f32,
) -> Color {
    match kind {
        BodyShader::Sun     => shade_sun(pos, normal, light_dir, time),
        BodyShader::Earth   => shade_earth(pos, normal, light_dir, time),
        BodyShader::Mars    => shade_mars(pos, normal, light_dir, time),
        BodyShader::Mercury => shade_mercury(pos, normal, light_dir, time),
    }
}

// ----------------- SOL (realista, emisivo, con ligera granulación) -----------------

fn shade_sun(_pos: Vec3, normal: Vec3, _light_dir: Vec3, time: f32) -> Color {
    let n = normal.normalize();
    let (lat, lon) = spherical_coords(n);

    // Centro casi blanco, transición a amarillo, borde anaranjado
    let center_color = rgb(255, 255, 245);
    let mid_color    = rgb(255, 225, 150);
    let edge_color   = rgb(255, 175, 70);

    let facing = clamp01(n.z * 0.5 + 0.5);
    let limb = 1.0 - facing; // 0 = centro, 1 = borde

    let base_mid = lerp_color(edge_color, mid_color, facing);
    let mut color = lerp_color(base_mid, center_color, facing.powf(1.6));

    // Granulación solar suave animada
    let granule_pattern =
        ((lon * 8.0 + time * 0.8).sin() * (lat * 14.0 + time * 0.6).cos()).abs();
    let granule_t = (granule_pattern - 0.4).max(0.0) * 0.4;
    let granule_color = rgb(255, 240, 190);
    color = lerp_color(color, granule_color, granule_t);

    // "Limb darkening": el borde un poco menos brillante
    let brightness = 1.0 - limb * 0.45;

    let r = (color.r as f32 * brightness).clamp(0.0, 255.0) as u8;
    let g = (color.g as f32 * brightness).clamp(0.0, 255.0) as u8;
    let b = (color.b as f32 * brightness).clamp(0.0, 255.0) as u8;

    Color { r, g, b, a: 255 }
}

// ----------------- TIERRA (rotando) -----------------

fn shade_earth(_pos: Vec3, normal: Vec3, light_dir: Vec3, time: f32) -> Color {
    let n = normal.normalize();
    let (lat, mut lon) = spherical_coords(n);

    // Rotación de la "textura" terrestre (giro sobre su eje)
    let earth_rotation_speed = 0.6;
    lon += time * earth_rotation_speed;

    // Océanos
    let ocean_deep = rgb(10, 40, 90);
    let ocean_shallow = rgb(30, 110, 170);
    let ocean_t = ((lat * 2.0).sin() * 0.5 + 0.5).powf(1.3);
    let mut color = lerp_color(ocean_deep, ocean_shallow, ocean_t);

    // Continentes
    let continents_pattern =
        (lon * 3.0 + 0.5).sin() * (lat * 4.0 - 0.2).cos() * 0.5 + 0.5;
    let continents_mask = clamp01((continents_pattern - 0.45) * 6.0);
    let land_low = rgb(30, 80, 30);
    let land_high = rgb(140, 110, 70);
    let land_mix = lerp_color(land_low, land_high, ((lat * 3.0).sin() * 0.5 + 0.5));
    color = lerp_color(color, land_mix, continents_mask);

    // Polos helados
    let poles = lat.abs().powf(3.0);
    let ice_color = rgb(220, 230, 240);
    color = lerp_color(color, ice_color, poles * 0.8);

    // Haze suave tipo nubes
    let soft_pattern =
        ((lon * 4.0 + time * 0.4).sin() * (lat * 7.0 + 0.5).cos()).abs();
    let soft_mask = clamp01((soft_pattern - 0.6) * 2.0);
    let soft_haze = rgb(210, 220, 230);
    color = lerp_color(color, soft_haze, soft_mask * 0.25);

    // Iluminación
    let diff = lambert(n, light_dir);
    let ambient = 0.25;
    let light_factor = clamp01(ambient + diff * 0.85);

    let r = (color.r as f32 * light_factor) as u8;
    let g = (color.g as f32 * light_factor) as u8;
    let b = (color.b as f32 * light_factor) as u8;

    Color { r, g, b, a: 255 }
}

// ----------------- MARTE (rotando) -----------------

fn shade_mars(_pos: Vec3, normal: Vec3, light_dir: Vec3, time: f32) -> Color {
    let n = normal.normalize();
    let (lat, mut lon) = spherical_coords(n);

    // Rotación marciana (un poco más lenta)
    let mars_rotation_speed = 0.35;
    lon += time * mars_rotation_speed;

    // Base rojiza
    let mars_dark = rgb(90, 40, 25);
    let mars_mid  = rgb(160, 70, 35);
    let mars_light = rgb(210, 120, 70);

    let base_t = ((lat * 1.5).sin() * 0.5 + 0.5).powf(1.5);
    let base_a = lerp_color(mars_dark, mars_mid, base_t);
    let mut color = lerp_color(base_a, mars_light, base_t * 0.6);

    // Zonas más oscuras
    let dark_pattern = ((lon * 3.5).sin() * (lat * 5.0).cos()).abs();
    let dark_mask = clamp01((dark_pattern - 0.6) * 3.0);
    let dark_color = rgb(60, 30, 25);
    color = lerp_color(color, dark_color, dark_mask * 0.7);

    // Casquetes polares suaves
    let poles = lat.abs().powf(4.0);
    let polar_color = rgb(220, 220, 230);
    color = lerp_color(color, polar_color, poles * 0.7);

    // Iluminación
    let diff = lambert(n, light_dir);
    let ambient = 0.18;
    let light_factor = clamp01(ambient + diff * 0.95);

    let r = (color.r as f32 * light_factor) as u8;
    let g = (color.g as f32 * light_factor * 0.9) as u8;
    let b = (color.b as f32 * light_factor * 0.85) as u8;

    Color { r, g, b, a: 255 }
}

// ----------------- MERCURIO (rotando rápido) -----------------

fn shade_mercury(_pos: Vec3, normal: Vec3, light_dir: Vec3, time: f32) -> Color {
    let n = normal.normalize();
    let (lat, mut lon) = spherical_coords(n);

    // Rotación rápida
    let mercury_rotation_speed = 1.0;
    lon += time * mercury_rotation_speed;

    // Base gris
    let rock_dark = rgb(40, 40, 45);
    let rock_light = rgb(130, 130, 135);
    let base_t = ((lon * 2.0).sin() * (lat * 3.0).cos() * 0.5 + 0.5).powf(1.2);
    let mut color = lerp_color(rock_dark, rock_light, base_t);

    // Cráteres suaves
    let crater_pattern = ((lon * 10.0).sin() * (lat * 12.0).cos()).abs();
    let crater_mask = clamp01((crater_pattern - 0.65) * 3.0);
    let crater_color = rgb(30, 30, 35);
    color = lerp_color(color, crater_color, crater_mask * 0.8);

    // Iluminación fuerte
    let diff = lambert(n, light_dir);
    let ambient = 0.1;
    let light_factor = clamp01(ambient + diff * 1.0);

    let r = (color.r as f32 * light_factor) as u8;
    let g = (color.g as f32 * light_factor) as u8;
    let b = (color.b as f32 * light_factor) as u8;

    Color { r, g, b, a: 255 }
}
