use core::cmp::Ordering;

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
