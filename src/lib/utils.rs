use alloc::vec::Vec;
use core::cmp::Ordering;
use crate::material::Material;
use crate::vec3::Vec3;

pub(crate) static EPSILON: f32 = 1e-3;

#[derive(PartialEq, Clone, Copy)]
pub struct OrderedFloat32 {
    pub val: f32,
}

impl OrderedFloat32 {
    pub fn new(val: f32) -> Self {
        OrderedFloat32 { val }
    }
}

impl Eq for OrderedFloat32 {}

impl PartialOrd for OrderedFloat32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for OrderedFloat32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.total_cmp(&other.val)
    }
}

pub type Vec3Idx = u32;
pub type MaterialIdx = u32;

#[derive(Debug, Clone)]
pub struct VecBuf {
    pub points: Vec<Vec3>,
}

impl VecBuf {
    pub fn push(&mut self, vec: Vec3) -> Vec3Idx {
        self.points.push(vec);
        (self.points.len() - 1) as Vec3Idx
    }

    pub fn load(&self, idx: Vec3Idx) -> &Vec3 {
        &self.points[idx as usize]
    }
}

#[derive(Debug, Clone)]
pub struct MaterialBuf {
    pub materials: Vec<Material>,
}

impl MaterialBuf {
    pub fn push(&mut self, material: Material) -> MaterialIdx {
        self.materials.push(material);
        (self.materials.len() - 1) as MaterialIdx
    }

    pub fn load(&self, idx: MaterialIdx) -> &Material {
        &self.materials[idx as usize]
    }
}
