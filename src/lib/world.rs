use crate::hit::Hit;
use crate::light::Light;
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

    pub fn get_hits<'a>(self: &Self, ray: &'a Ray) -> Vec<Hit<'a>> {
        let mut hits = Vec::new();
        for object in self.objects.iter() {
            let mut object_hits = object.get_hits(ray, object.clone());
            hits.append(&mut object_hits);
        }
        hits
    }
}
