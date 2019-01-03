use crate::vec::Vec3;

#[derive(PartialEq, Debug)]
pub struct Ray<'a, 'b> {
    a: &'a Vec3,
    b: &'b Vec3,
}

impl<'a, 'b> Ray<'a, 'b> {
    pub fn new(vec_a: &'a Vec3, vec_b: &'b Vec3) -> Ray<'a, 'b> {
        Ray { a: vec_a, b: vec_b }
    }

    pub fn origin(&self) -> &Vec3 {
        self.a
    }

    pub fn direction(&self) -> &Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin() + &(self.direction() * t)
    }

}

#[test]
fn point_at_parameter() {
    let vector1 = Vec3::new(0.0, 1.0, 0.0);
    let vector2 = Vec3::new(3.0, 2.0, 1.0);
    let ray1 = Ray::new(&vector1, &vector2);

    assert_eq!(Vec3::new(6.0, 5.0, 2.0), ray1.point_at_parameter(2.0))
}
