use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub from: Vec3,
    pub dir: Vec3,
    pub inv_dir: Vec3,
    pub sign_x: usize,
    pub sign_y: usize,
    pub sign_z: usize,
}

impl Ray {
    pub fn new(from: Vec3, dir: Vec3) -> Ray {
        let sign_x = (dir.x < 0.0) as usize;
        let sign_y = (dir.y < 0.0) as usize;
        let sign_z = (dir.z < 0.0) as usize;
        Ray {
            from,
            dir,
            inv_dir: Vec3::new(dir.x.recip(), dir.y.recip(), dir.z.recip()),
            sign_x,
            sign_y,
            sign_z,
        }
    }
}
