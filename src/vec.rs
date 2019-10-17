use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, Index};
use rand::Rng;
use rand::seq::SliceRandom;
use std::iter::Sum;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub enum Axis { X, Y, Z }

impl Axis {
    pub fn xyz() -> [Axis; 3] {
        [Axis::X, Axis::Y, Axis::Z]
    }

    pub fn random<R: Rng + ?Sized>(rnd: &mut R) -> &'static Axis {
        [Axis::X, Axis::Y, Axis::Z].choose(rnd).unwrap()
    }
}

impl Index<&Axis> for V3 {
    type Output = f64;

    fn index(&self, axis: &Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl V3 {
    pub fn new(x: f64, y: f64, z: f64) -> V3 {
        V3 { x, y, z }
    }

    pub fn ones() -> V3 {
        V3::new(1.0, 1.0, 1.0)
    }
    pub fn zeros() -> V3 {
        V3::new(0.0, 0.0, 0.0)
    }

    pub fn sqr_length(self) -> f64 {
        self.dot(self)
    }
    pub fn length(&self) -> f64 {
        self.sqr_length().sqrt()
    }

    pub fn dot(&self, other: V3) -> f64 {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
    }
    pub fn cross(&self, other: V3) -> V3 {
        V3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit(&self) -> V3 {
        let scale = 1.0 / self.length();
        V3 {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }

    pub fn reflect(&self, normal: V3) -> V3 {
        *self - 2.0 * self.dot(normal) * normal
    }
}

impl From<[f64; 3]> for V3 {
    fn from(vec: [f64; 3]) -> Self {
        V3 { x: vec[0], y: vec[1], z: vec[2] }
    }
}

impl From<V3> for [f64; 3] {
    fn from(v3: V3) -> Self {
        [v3.x, v3.y, v3.z]
    }
}

impl Add for V3 {
    type Output = V3;

    fn add(self, other: V3) -> V3 {
        V3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for V3 {
    type Output = V3;

    fn add(self, other: f64) -> V3 {
        V3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, other: V3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sum for V3{
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(V3::zeros(), V3::add)
    }
}

impl Mul<V3> for f64 {
    type Output = V3;

    fn mul(self, other: V3) -> V3 {
        V3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<V3> for f32 {
    type Output = V3;

    fn mul(self, other: V3) -> V3 {
        V3 {
            x: (self * other.x as f32) as f64,
            y: (self * other.y as f32) as f64,
            z: (self * other.z as f32) as f64,
        }
    }
}

impl Mul<V3> for V3 {
    type Output = V3;

    fn mul(self, other: V3) -> V3 {
        V3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}


impl Div<f64> for V3 {
    type Output = V3;

    fn div(self, other: f64) -> V3 {
        V3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}


impl Neg for V3 {
    type Output = V3;

    fn neg(self) -> V3 {
        V3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


impl Sub for V3 {
    type Output = V3;

    fn sub(self, other: V3) -> V3 {
        V3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl Sub<f64> for V3 {
    type Output = V3;

    fn sub(self, rhs: f64) -> V3 {
        V3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}


#[cfg(test)]
mod test {
    use super::V3;

    #[test]
    fn add() {
        assert_eq!(
            V3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            } + V3 {
                x: 2.0,
                y: 1.0,
                z: 2.0,
            },
            V3 {
                x: 3.0,
                y: 1.0,
                z: 4.0,
            }
        );
    }

    #[test]
    fn add_assign() {
        let mut x = V3::new(0.0, 0.0, 0.0);
        let y = V3::new(1.0, 2.0, 3.0);
        x += y;
        assert_eq!(
            x,
            V3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );
    }

    #[test]
    fn cross() {
        assert_eq!(
            V3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            }
                .cross(V3 {
                    x: 2.0,
                    y: 1.0,
                    z: 2.0,
                }),
            V3 {
                x: -2.0,
                y: 2.0,
                z: 1.0,
            }
        );
    }

    #[test]
    fn dot() {
        assert_eq!(
            V3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            }
                .dot(V3 {
                    x: 2.0,
                    y: 1.0,
                    z: 2.0,
                }),
            6.0
        );
    }

    #[test]
    fn length() {
        let v = V3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = V3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.length(), 3.0);
        assert_eq!(u.length(), 1.0);
    }

    #[test]
    fn sqr_length() {
        let v = V3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = V3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.sqr_length(), 9.0);
        assert_eq!(u.sqr_length(), 1.0);
    }

    #[test]
    fn mul() {
        assert_eq!(
            3.0 * V3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            V3 {
                x: 3.0,
                y: 6.0,
                z: 9.0,
            }
        );
    }

    #[test]
    fn hadamard() {
        let lhs = V3::new(1.0, 1.0, 1.0);
        let rhs = V3::new(2.0, 3.0, 4.0);
        assert_eq!(lhs * rhs, V3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn neg() {
        assert_eq!(
            -V3 {
                x: 1.0,
                y: -2.0,
                z: 3.0,
            },
            V3 {
                x: -1.0,
                y: 2.0,
                z: -3.0,
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            V3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            } - V3 {
                x: 2.0,
                y: 1.0,
                z: 2.0,
            },
            V3 {
                x: -1.0,
                y: -1.0,
                z: 0.0,
            }
        );
    }
}