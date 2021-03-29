use std::f32::consts::PI;
use std::ptr;

use float_eq::float_eq;

use crate::color::Color;
use crate::config;
use crate::hit::Hit;
use crate::object::{get_closest, Object};
use crate::ray::Ray;
use crate::world::World;

const DIRECT_LIGHT_CEIL: f32 = 0.01;
const LIGHT_INCREASE: f32 = 30.0;
const REFRACTIVE_INDEX: f32 = 1.5;

pub struct RayLauncher<'a> {
    world: &'a World,
}

impl<'a> RayLauncher<'a> {
    pub fn new(world: &'a World) -> Self {
        RayLauncher { world }
    }

    pub fn launch_primary_ray(self: &Self, ray: &Ray) -> Color {
        self.launch_ray(ray, None, 0)
    }

    fn launch_ray(
        self: &Self,
        ray: &Ray,
        emitting_object: Option<&dyn Object>,
        depth: usize,
    ) -> Color {
        if depth > config::MAX_DEPTH {
            return Color::new_black();
        }

        let hits = self.world.get_hits(ray);
        let hit_opts = hits
            .into_iter()
            .filter(|hit| hit.distance_ratio > 0.0 && !is_self_hit(hit, emitting_object))
            .fold(None, get_closest);
        match hit_opts {
            Some(hit) => self.calc_color(&ray, emitting_object, &hit, depth),
            None => self.calc_light_color(&ray, emitting_object),
        }
    }

    fn calc_color(
        self: &Self,
        camera_ray: &Ray,
        emitting_object: Option<&dyn Object>,
        hit: &Hit,
        depth: usize,
    ) -> Color {
        let mut color = Color::new_black();
        let transmited_color = self.calc_transmited_color(hit);
        color.add_into(&transmited_color);
        let light_color = self.calc_light_color(camera_ray, emitting_object);
        color.add_into(&light_color);
        if hit.object.has_reflexion() {
            let reflexion_color = self.calc_reflexion_color(camera_ray, hit, depth + 1);
            color.add_into(&reflexion_color);
        }
        if hit.object.has_refraction() {
            let refraction_color = self.calc_refraction_color(camera_ray, hit, depth + 1);
            color.add_into(&refraction_color);
        }
        color
    }

    fn calc_light_color(
        self: &Self,
        camera_ray: &Ray,
        emitting_object: Option<&dyn Object>,
    ) -> Color {
        let mut light_color = Color::new_black();
        let normalized_ray = camera_ray.direction.get_normalised();

        for light in self.world.lights.iter() {
            let direction = light.get_direction(&camera_ray.origin);
            let normalized_direction = direction.get_normalised();
            let angle_ratio = normalized_direction.dot(&normalized_ray);
            if angle_ratio < 0.0 {
                // light is behind the camera, no need to launch ray
                continue;
            }

            let angle_ratio = angle_ratio.powf(LIGHT_INCREASE);
            if angle_ratio < DIRECT_LIGHT_CEIL {
                // The resulting light would be too little, no need to compute
                continue;
            }
            let light_ray = Ray::new(camera_ray.origin.clone(), direction);

            let hits = self.world.get_hits(&light_ray);
            if hits
                .iter()
                .filter(|light_hit| {
                    light.is_touching(light_hit) && !is_self_hit(light_hit, emitting_object)
                })
                .count()
                > 0
            {
                // Something is between the camera and the light
                continue;
            }

            let intensity = light.get_intensity(&light_ray.direction);
            let ratio = intensity * angle_ratio;
            let current_light_color = light.get_color().apply_ratio(ratio);
            light_color.add_into(&current_light_color);
        }

        light_color
    }

    fn calc_transmited_color(self: &Self, hit: &Hit) -> Color {
        let mut light_color = Color::new_black();

        let hit_position = hit.get_position();
        let hit_normal = hit.get_normal();

        for light in self.world.lights.iter() {
            let direction = light.get_direction(hit_position);
            if direction.dot(&hit_normal) < 0.0 {
                // light is behind the object, no need to launch ray
                continue;
            }
            let normalized_direction = direction.get_normalised();
            let light_ray = Ray::new(hit_position.clone(), direction);

            let hits = self.world.get_hits(&light_ray);
            if hits
                .iter()
                .filter(|light_hit| {
                    light.is_touching(light_hit)
                        && !is_self_hit(light_hit, Some(hit.object.as_ref()))
                })
                .count()
                > 0
            {
                // Something is between the hit and the light
                continue;
            }

            let normal_ratio = normalized_direction.dot(&hit_normal).abs();

            let ratio = normal_ratio * light.get_intensity(&light_ray.direction) / PI
                * hit.object.get_transmission();
            let current_light_color = light.get_color().apply_ratio(ratio);
            light_color.add_into(&current_light_color);
        }

        light_color.multiply(hit.object.get_color())
    }

    fn calc_reflexion_color(
        self: &Self,
        incident_ray: &Ray,
        incident_hit: &Hit,
        depth: usize,
    ) -> Color {
        let hit_position = incident_hit.get_position();
        let hit_normal = incident_hit.get_normal();
        let incident_direction = incident_ray.direction.get_normalised();

        let dot = hit_normal.dot(&incident_direction);
        let reflected_direction = incident_direction.diff(&hit_normal.multiply(2.0 * dot));

        let reflected_ray = Ray::new(hit_position.clone(), reflected_direction);
        self.launch_ray(&reflected_ray, Some(incident_hit.object.as_ref()), depth)
    }

    fn calc_refraction_color(
        self: &Self,
        incident_ray: &Ray,
        incident_hit: &Hit,
        depth: usize,
    ) -> Color {
        let hit_position = incident_hit.get_position();
        let mut hit_normal = incident_hit.get_normal();
        let incident_direction = incident_ray.direction.get_normalised();

        let mut lol = None;

        let mut refration_ratio = 1.0 / REFRACTIVE_INDEX;
        let mut dot_normal_incident = incident_direction.dot(&hit_normal);
        if incident_hit.is_outside() {
            dot_normal_incident = -dot_normal_incident;
        } else {
            refration_ratio = 1.0 / refration_ratio;
            hit_normal = lol.get_or_insert(hit_normal.get_reverse());
        }

        let k = 1.0
            - refration_ratio * refration_ratio * (1.0 - dot_normal_incident * dot_normal_incident);

        if k > 0.0 {
            let right_coeff = refration_ratio * dot_normal_incident - f32::sqrt(k);
            let refracted_direction = incident_direction
                .multiply(refration_ratio)
                .add(&hit_normal.multiply(right_coeff));
            let refracted_ray = Ray::new(hit_position.clone(), refracted_direction);
            return self.launch_ray(&refracted_ray, Some(incident_hit.object.as_ref()), depth);
        }
        Color::new_black()
    }
}

fn is_self_hit(hit: &Hit, initial_object_opts: Option<&dyn Object>) -> bool {
    match initial_object_opts {
        Some(initial_object) => {
            if !ptr::eq(hit.object.as_ref(), initial_object) {
                return false;
            }
            float_eq!(hit.distance_ratio, 0.0, abs <= 0.000_1)
        }
        None => false,
    }
}
