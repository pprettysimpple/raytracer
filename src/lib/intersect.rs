use super::material::Material;
use super::vec3::Vec3;

pub trait Intersect {
    ///
    ///
    /// # Arguments
    ///
    /// * `from`: ray base point
    /// * `dir`: direction of the ray
    ///
    /// returns: Option<(Vec3, Vec3, Material)>
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracer::entity::sphere::Sphere;
    /// use raytracer::intersect::Intersect;
    /// use raytracer::vec3::Vec3;
    /// let object = Sphere::new(Default::default(), 1.0, Default::default());
    /// let from = Vec3::new(-10.0, 0.0, 0.0);
    /// let dir = Vec3::new(1.0, 0.0, 0.0);
    /// if let Some((hit, normal, material)) = object.ray_intersect(&from, &dir) {
    ///     // hit      -- the place on the object where hit happened
    ///     // normal   -- the normal vector for this hit
    ///     // material -- material at the hit point
    /// } else {
    ///     // there was no hit
    /// }
    /// ```
    fn ray_intersect(&self, from: &Vec3, dir: &Vec3) -> Option<(Vec3, Vec3, Material)>;
}
