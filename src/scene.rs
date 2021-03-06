use std::f64;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::aabb::{AABB, BoundingBox, surrounding_box, BVH};
use crate::intersectable::{Intersection};
use crate::texture::{CheckerTexture, NoiseTexture, TextureCoords};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> Material {
        self.material
    }
}

impl BoundingBox for Sphere {
    fn bounding_box(&self) -> AABB {
        return AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    t0: f64,
    t1: f64,
    radius: f64,
    material: Material,
}
impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, radius: f64, material: Material, t0: f64, t1: f64) -> MovingSphere {
        MovingSphere { center0, center1, t0, t1, radius, material }
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.t0) / (self.t1 - self.t0)) * (self.center1 - self.center0)
    }

    pub fn radius(&self) -> f64 { self.radius }
}

impl BoundingBox for MovingSphere {
    fn bounding_box(&self) -> AABB {
        let start_bb = AABB::new(
            self.center(self.t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(self.t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let finish_bb = AABB::new(
            self.center(self.t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(self.t1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        surrounding_box(&start_bb, &finish_bb)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SceneItem {
    Sphere(Sphere),
    MovingSphere(MovingSphere)
}
impl SceneItem {
    pub fn material(&self) -> Material {
        match self {
            SceneItem::Sphere(ref s) => s.material(),
            SceneItem::MovingSphere(ref s) => s.material()
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        match self {
            SceneItem::Sphere(ref s) => s.center(),
            SceneItem::MovingSphere(ref s) => s.center(time)
        }
    }
}

impl BoundingBox for SceneItem {
    fn bounding_box(&self) -> AABB {
        match self {
            SceneItem::Sphere(ref s) => s.bounding_box(),
            SceneItem::MovingSphere(ref s) => s.bounding_box()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Surface {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32 }
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Coloration,
    pub albedo: f32,
    pub surface: Surface
}

#[derive(Copy, Clone, Debug)]
pub enum Coloration {
    Color(Color),
    CheckerTexture(CheckerTexture),
    NoiseTexture(NoiseTexture)
}

impl Coloration {
    pub fn color(&self, texture_coords: &TextureCoords, ray_point: &Vec3) -> Color {
        match *self {
            Coloration::Color(c) => c,
            Coloration::CheckerTexture(t) => t.get_color(&texture_coords, &ray_point),
            Coloration::NoiseTexture(t) => t.get_color(&texture_coords, &ray_point)
        }
    }
}

pub struct Scene {
    _items: Vec<SceneItem>,
    bvh: BVH
}

impl Scene {
    pub fn new(mut items: Vec<SceneItem>) -> Scene {
        let scene_bvh = BVH::new(&mut items[..]);
        Scene {
            _items: items,
            bvh: scene_bvh
        }
    }

    pub fn trace(&self, r: &Ray) -> Option<Intersection> {
        self.bvh.intersect(&r)
    }
}
