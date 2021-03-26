use crate::light::Light;
use crate::object::Hit;
use crate::object::Object;
use crate::ray::Ray;
use std::rc::Rc;

pub struct World {
    objects: Vec<Rc<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
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

    pub fn add_light(self: &mut Self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn get_closest_hit(self: &Self, ray: &Ray) -> Option<Hit> {
        let hits = self.get_hits(ray);

        if hits.len() == 0 {
            return None;
        }
        hits.into_iter()
            .filter(|hit| hit.distance_ratio > 0.0)
            .fold(None, get_closest)
    }

    pub fn get_hits(self: &Self, ray: &Ray) -> Vec<Hit> {
        let mut hits = Vec::new();
        for object in self.objects.iter() {
            let mut object_hits = object.get_hits(ray, object.clone());
            hits.append(&mut object_hits);
        }
        hits
    }
}

fn get_closest(left: Option<Hit>, right: Hit) -> Option<Hit> {
    match &left {
        None => Some(right),
        Some(left_hit) => {
            if left_hit.distance_ratio < right.distance_ratio {
                left
            } else {
                Some(right)
            }
        }
    }
}
