use crate::light::spotlight::SpotLight;
use crate::object::Intersection;
use crate::object::Object;
use crate::ray::Ray;
use std::rc::Rc;

pub struct World {
    objects: Vec<Rc<dyn Object>>,
    pub lights: Vec<Rc<SpotLight>>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(self: &mut Self, object: Rc<dyn Object>) {
        self.objects.push(object);
    }

    pub fn add_light(self: &mut Self, light: Rc<SpotLight>) {
        self.lights.push(light);
    }

    pub fn get_closest_intersection(self: &Self, ray: &Ray) -> Option<Intersection> {
        let intersections = self.get_intersections(ray);

        if intersections.len() == 0 {
            return None;
        }
        intersections
            .into_iter()
            .filter(|intersection| intersection.distance_ratio > 0.0)
            .fold(None, get_closest)
    }

    pub fn get_intersections(self: &Self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for object in self.objects.iter() {
            let mut object_intersections = object.get_intersections(ray, object.clone());
            intersections.append(&mut object_intersections);
        }
        intersections
    }
}

fn get_closest(left: Option<Intersection>, right: Intersection) -> Option<Intersection> {
    match &left {
        None => Some(right),
        Some(left_intersection) => {
            if left_intersection.distance_ratio < right.distance_ratio {
                left
            } else {
                Some(right)
            }
        }
    }
}
