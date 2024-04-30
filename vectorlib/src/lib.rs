use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn r(&self) -> f64 {
        self.x
    }

    pub fn g(&self) -> f64 {
        self.y
    }

    pub fn b(&self) -> f64 {
        self.z
    }

    pub fn dot(&self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn make_unit_vector(&self) -> Vector3 {
        if self.length().abs() < 1e-5 {
            panic!("Cannot normalize a zero-length vector");
        } else {
            *self / self.length()
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = *self + other;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        *self = *self - other;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = *self * scalar;
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, scalar: f64) {
        *self = *self / scalar;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_and_get_value() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.x(), 1.0);
        assert_eq!(v1.y(), 2.0);
        assert_eq!(v1.z(), 3.0);

        assert_eq!(v1.r(), 1.0);
        assert_eq!(v1.g(), 2.0);
        assert_eq!(v1.b(), 3.0);
    }

    #[test]
    fn test_add() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;

        assert_eq!(v3.x(), 5.0);
        assert_eq!(v3.y(), 7.0);
        assert_eq!(v3.z(), 9.0);

        v1 += v2;

        assert_eq!(v1.x(), 5.0);
        assert_eq!(v1.y(), 7.0);
        assert_eq!(v1.z(), 9.0);

        println!("{:?}", v2);
    }

    #[test]
    fn test_sub() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v1 - v2;

        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), -3.0);
        assert_eq!(v3.z(), -3.0);

        v1 -= v2;

        assert_eq!(v1.x(), -3.0);
        assert_eq!(v1.y(), -3.0);
        assert_eq!(v1.z(), -3.0);
    }

    #[test]
    fn test_scale_mul() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;

        assert_eq!(v2.x(), 2.0);
        assert_eq!(v2.y(), 4.0);
        assert_eq!(v2.z(), 6.0);

        v1 *= 2.0;

        assert_eq!(v1.x(), 2.0);
        assert_eq!(v1.y(), 4.0);
        assert_eq!(v1.z(), 6.0);
    }

    #[test]
    fn test_scale_divide() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = v1 / 2.0;

        assert_eq!(v2.x(), 0.5);
        assert_eq!(v2.y(), 1.0);
        assert_eq!(v2.z(), 1.5);

        v1 /= 2.0;

        assert_eq!(v1.x(), 0.5);
        assert_eq!(v1.y(), 1.0);
        assert_eq!(v1.z(), 1.5);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        let dot = v1.dot(v2);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);

        let cross = v1.cross(v2);

        assert_eq!(cross.x(), 0.0);
        assert_eq!(cross.y(), 0.0);
        assert_eq!(cross.z(), 1.0);
    }

    #[test]
    fn test_length() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let length = v1.length();
        assert_eq!(length, 14.0_f64.sqrt());
        assert_eq!(v1.length_squared(), 14.0);
        assert_eq!(v1.make_unit_vector().length(), 1.0);
    }
}
