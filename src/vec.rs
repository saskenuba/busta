use std::ops::Add;

#[derive(Debug)]
pub struct Vec3 {
    pub vector: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            vector: [e0, e1, e2],
        }
    }
    pub fn dot(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            vector: [
                v1.vector[0] * v2.vector[0],
                v1.vector[1] * v2.vector[1],
                v1.vector[2] * v2.vector[2],
            ],
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        // iter both, and for each element compare, im getting gud at this
        self.vector.iter().zip(other.vector.iter()).all(|(a,b)| a == b)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            vector: [
                self.vector[0] + other.vector[0],
                self.vector[1] + other.vector[1],
                self.vector[2] + other.vector[2],
            ],
        }
    }
}

#[test]
fn vec3() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(2.0, 3.0, 4.0);

    assert_eq!(Vec3::dot(vector1, vector2), Vec3::new(2.0, 6.0, 12.0));
}
