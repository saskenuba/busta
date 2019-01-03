use std::ops::*;

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
        let mut result: [f64; 3] = [0.0; 3];
        for ((resultval, x), y) in result.iter_mut().zip(&v1.vector).zip(&v2.vector) {
            *resultval = x * y;
        }
        Vec3 { vector: result }
    }
    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            vector: [
                v1.vector[1] * v2.vector[2] - v1.vector[2] * v2.vector[1],
                -(v1.vector[0] * v2.vector[2] - v1.vector[2] * v2.vector[0]),
                v1.vector[0] * v2.vector[1] - v1.vector[1] * v2.vector[0],
            ],
        }
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        (self.x()).powf(2.0) + (self.y()).powf(2.0) + (self.z()).powf(2.0)
    }

    pub fn x(&self) -> f64 {
        self.vector[0]
    }
    pub fn y(&self) -> f64 {
        self.vector[1]
    }
    pub fn z(&self) -> f64 {
        self.vector[2]
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        // iter both, and for each element compare, im getting gud at this
        self.vector
            .iter()
            .zip(other.vector.iter())
            .all(|(a, b)| a == b)
    }
}

macro_rules! overload_vec3 {
    ($oper:tt, $oper_name:ident, $oper_name_lowercase:ident, Vec3, 0) => {
        impl $oper_name<Vec3> for Vec3 {
            type Output = Vec3;
            fn $oper_name_lowercase(self, other: Vec3) -> Vec3 {
                Vec3 {
                    vector: [
                        self.vector[0] $oper other.vector[0],
                        self.vector[1] $oper other.vector[1],
                        self.vector[2] $oper other.vector[2],
                    ],
                }
            }
        }
        impl<'a, 'b> $oper_name<&'b Vec3> for &'b Vec3 {
            type Output = Vec3;
            fn $oper_name_lowercase(self, other: &'b Vec3) -> Vec3 {
                Vec3 {
                    vector: [
                        self.vector[0] $oper other.vector[0],
                        self.vector[1] $oper other.vector[1],
                        self.vector[2] $oper other.vector[2],
                    ],
                }
            }
        }
    };
    ($oper:tt, $oper_name:ident, $oper_name_lowercase:ident, $custom_sig:ty, 0) => {
        impl $oper_name<$custom_sig> for Vec3 {
            type Output = Vec3;
            fn $oper_name_lowercase(self, other: $custom_sig) -> Vec3 {
                Vec3 {
                    vector: [
                        self.vector[0] $oper other,
                        self.vector[1] $oper other,
                        self.vector[2] $oper other,
                    ],
                }
            }
        }
        impl<'a> $oper_name<$custom_sig> for &'a Vec3 {
            type Output = Vec3;
            fn $oper_name_lowercase(self, other: $custom_sig) -> Vec3 {
                Vec3 {
                    vector: [
                        self.vector[0] $oper other,
                        self.vector[1] $oper other,
                        self.vector[2] $oper other,
                    ],
                }
            }
        }
    };
    ($oper:tt, $oper_name:ident, $oper_name_lowercase:ident, Vec3, 1) => {
        impl $oper_name<Vec3> for Vec3 {
            fn $oper_name_lowercase(&mut self, other: Vec3) {
                self.vector[0] $oper other.vector[0];
                self.vector[1] $oper other.vector[1];
                self.vector[2] $oper other.vector[2];
            }
        }
    };
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            vector: [-self.vector[0], -self.vector[1], -self.vector[2]],
        }
    }
}

// Here we overload all binary operators, macros rocks!
overload_vec3!(+, Add, add, Vec3, 0);
overload_vec3!(-, Sub, sub, Vec3, 0);
overload_vec3!(*, Mul, mul, Vec3, 0);
overload_vec3!(/, Div, div, Vec3, 0);
overload_vec3!(*, Mul, mul, f64, 0);
overload_vec3!(/, Div, div, f64, 0);
overload_vec3!(+=, AddAssign, add_assign, Vec3, 1);
overload_vec3!(-=, SubAssign, sub_assign, Vec3, 1);
overload_vec3!(*=, MulAssign, mul_assign, Vec3, 1);
overload_vec3!(/=, DivAssign, div_assign, Vec3, 1);

#[test]
fn dot() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(Vec3::dot(vector1, vector2), Vec3::new(2.0, 6.0, 12.0));
}

#[test]
fn add() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(vector1 + vector2, Vec3::new(3.0, 5.0, 7.0));

    let mut vector3 = Vec3::new(1.0, 2.0, 3.0);
    let vector4 = Vec3::new(2.0, 3.0, 4.0);
    vector3 += vector4;
    assert_eq!(vector3, Vec3::new(3.0, 5.0, 7.0));
}

#[test]
fn sub() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(vector1 - vector2, Vec3::new(-1.0, -1.0, -1.0));

    let mut vector3 = Vec3::new(1.0, 2.0, 3.0);
    let vector4 = Vec3::new(2.0, 3.0, 4.0);
    vector3 -= vector4;
    assert_eq!(vector3, Vec3::new(-1.0, -1.0, -1.0));
}

#[test]
fn neg() {
    let vector1 = Vec3::new(-1.0, 2.0, 3.0);
    assert_eq!(-vector1, Vec3::new(1.0, -2.0, -3.0));

}

#[test]
fn mul() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(2.0, 3.0, 4.0);
    let vector3 = Vec3::new(2.0, 20.0, -7.5);
    assert_eq!(vector1 * vector2, Vec3::new(2.0, 6.0, 12.0));
    assert_eq!(vector3 * 3.0, Vec3::new(6.0, 60.0, -22.5));

    let mut vector3 = Vec3::new(1.0, 2.0, 3.0);
    let vector4 = Vec3::new(2.0, 3.0, 4.0);
    vector3 *= vector4;
    assert_eq!(vector3, Vec3::new(2.0, 6.0, 12.0));
}

#[test]
fn div() {
    let vector1 = Vec3::new(5.0, 10.0, 2.5);
    let vector2 = Vec3::new(10.0, 40.0, 5.0);
    let vector3 = Vec3::new(2.0, 10.0, 3.4);
    assert_eq!(vector2 / vector1, Vec3::new(2.0, 4.0, 2.0));
    assert_eq!(vector3 / 2.0, Vec3::new(1.0, 5.0, 1.7));

    let mut vector3 = Vec3::new(1.0, 2.0, 3.0);
    let vector4 = Vec3::new(2.0, 2.0, 4.0);
    vector3 /= vector4;
    assert_eq!(vector3, Vec3::new(0.5, 1.0, 0.75));

}

#[test]
fn cross() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(Vec3::cross(vector1, vector2), Vec3::new(-1.0, 2.0, -1.0));
}

#[test]
fn length() {
    let vector1 = Vec3::new(10.0, 10.0, 10.0);
    assert_eq!(vector1.squared_length(), 300.0);
    assert_eq!(((vector1.length() * 10000.0).round() / 10000.0), 17.3205);
}

#[test]
fn squared_length() {
    let vector1 = Vec3::new(10.0, 10.0, 10.0);
    assert_eq!(vector1.squared_length(), 300.0);
}
