use crate::object::Intersection;
use crate::object::Object;
use crate::ray::Ray;

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }

    pub fn get_closest_intersection(self: &Self, ray: &Ray) -> Option<Intersection> {
        let mut intersections = Vec::new();
        for object in self.objects.iter() {
            let mut object_intersections = object.get_intersections(ray);
            intersections.append(&mut object_intersections);
        }

        if intersections.len() == 0 {
            return None;
        }
        intersections
            .into_iter()
            .filter(|intersection| intersection.distance_ratio > 0.0)
            .fold(None, get_closest)
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
