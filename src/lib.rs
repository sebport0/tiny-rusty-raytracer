use std::{fmt, ops};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn new_zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn cross(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let norm = self.norm();

        Vector3 {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Dot product between two Vector3.
impl ops::Mul<Vector3> for Vector3 {
    type Output = f64;

    fn mul(self, v: Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, k: f64) -> Self {
        Self {
            x: k * self.x,
            y: k * self.y,
            z: k * self.z,
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, v: Self) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
        }
    }
}

impl ops::Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range."),
        }
    }
}

impl ops::IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range."),
        }
    }
}

pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn ray_intersect(&self, orig: &Vector3, dir: &Vector3, t0: &f64) -> bool {
        let l = self.center - *orig;
        let tca = l * *dir;
        let d2 = l * l - tca * tca;
        let r2 = self.radius.powi(2);

        if d2 > r2 {
            return false;
        };

        let thc = (r2 - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0.0 {
            t0 = t1;
        }

        if t0 < 0.0 {
            return false;
        }

        true
    }
}

pub fn cast_ray(orig: &Vector3, dir: &Vector3, sphere: &Sphere) -> Vector3 {
    let sphere_dist = std::f64::MAX;

    if !sphere.ray_intersect(orig, dir, &sphere_dist) {
        return Vector3::new(0.2, 0.7, 0.8); // background color.
    }

    Vector3::new(0.4, 0.4, 0.3)
}

#[cfg(test)]
mod tests_vector3 {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);

        let w = Vector3::new(-5.0, 544.0, 2435.234);
        assert_eq!(w.x, -5.0);
        assert_eq!(w.y, 544.0);
        assert_eq!(w.z, 2435.234);
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector3::new(5.0, 10.0, -25.0);
        let v2 = Vector3::new(1.0, 0.0, 4.0);
        let expected = Vector3::new(40.0, -45.0, -10.0);

        assert_eq!(v1.cross(&v2), expected);
    }

    #[test]
    fn test_vector_norm() {
        let v = Vector3::new(-2.0, 0.0, 0.0);
        let expected = 2.0;

        assert_eq!(v.norm(), expected);
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector3::new(5.0, 10.0, -25.0);
        let v2 = Vector3::new(1.0, 0.0, 4.0);
        let expected = -95.0;

        assert_eq!(v1 * v2, expected);
    }

    #[test]
    fn test_vector_normalization() {
        let v = Vector3::new(1.0, 1.0, -1.0);
        let expected = Vector3::new(0.5773502691896258, 0.5773502691896258, -0.5773502691896258);

        assert_eq!(v.normalize(), expected);
    }

    #[test]
    fn test_vector_multiplication_by_scalar_right() {
        let v1 = Vector3::new(5.0, 10.0, -25.0);
        let scalar = 5.0;
        let expected = Vector3::new(25.0, 50.0, -125.0);

        assert_eq!(v1 * scalar, expected);
    }

    #[test]
    fn test_vector_multiplication_by_scalar_left() {
        let v1 = Vector3::new(5.0, 10.0, -25.0);
        let scalar = 5.0;
        let expected = Vector3::new(25.0, 50.0, -125.0);

        assert_eq!(scalar * v1, expected);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector3::new(5.0, 10.0, -25.0);
        let v2 = Vector3::new(-5.0, 1.0, 0.0);
        let expected = Vector3::new(0.0, 11.0, -25.0);

        assert_eq!(v1 + v2, expected);
    }

    #[test]
    fn test_vector_substraction() {
        let v1 = Vector3::new(5.0, 10.0, -25.0);
        let v2 = Vector3::new(-5.0, 1.0, 0.0);
        let expected = Vector3::new(10.0, 9.0, -25.0);

        assert_eq!(v1 - v2, expected);
    }

    #[test]
    fn vector_negation() {
        let v1 = Vector3::new(5.0, 10.0, -25.0);
        let expected = Vector3::new(-5.0, -10.0, 25.0);

        assert_eq!(-v1, expected);
    }

    #[test]
    fn test_vector_index_access() {
        let v = Vector3::new(0.0, 1.0, 2.0);
        assert_eq!(v[0], 0.0);
        assert_eq!(v[1], 1.0);
        assert_eq!(v[2], 2.0);
    }

    #[test]
    #[should_panic]
    fn test_vector_panics_when_index_out_of_range() {
        let v = Vector3::new(0.0, 1.0, 2.0);
        v[3];
    }

    #[test]
    fn test_vector_index_access_assign() {
        let mut v = Vector3::new(0.0, 1.0, 2.0);

        v[0] = -10.0;
        assert_eq!(v[0], -10.0);

        v[1] = 5.0;
        assert_eq!(v[1], 5.0);

        v[2] = 35.555;
        assert_eq!(v[2], 35.555);
    }

    #[test]
    #[should_panic]
    fn test_vector_panics_when_try_to_assign_to_index_out_of_range() {
        let mut v = Vector3::new(0.0, 1.0, 2.0);
        v[5] = 12.0;
    }
}

// TODO: Sphere tests!
