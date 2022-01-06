use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use std::slice::SliceIndex;

#[derive(Debug, Default, Clone, Copy)]
pub struct V3 {
    v: [f64; 3],
}

impl V3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { v: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.v[1]
    }

    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.v[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

impl<Idx: SliceIndex<[f64]>> Index<Idx> for V3 {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        Index::index(&self.v, index)
    }
}

impl<Idx: SliceIndex<[f64]>> IndexMut<Idx> for V3 {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.v, index)
    }
}

impl Neg for V3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            v: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl Eq for V3 {}

impl PartialEq for V3 {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl Add for V3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add<f64> for V3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        let rhs = V3::new(rhs, rhs, rhs);
        self + rhs
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, rhs: Self) {
        *self.x_mut() += rhs.x();
        *self.y_mut() += rhs.y();
        *self.z_mut() += rhs.z();
    }
}

impl Sub for V3 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Sub<f64> for V3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        let rhs = V3::new(rhs, rhs, rhs);
        self - rhs
    }
}

impl SubAssign for V3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self.x_mut() -= rhs.x();
        *self.y_mut() -= rhs.y();
        *self.z_mut() -= rhs.z();
    }
}

impl Mul for V3 {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<f64> for V3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs = V3::new(rhs, rhs, rhs);
        self * rhs
    }
}

impl MulAssign for V3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self.x_mut() *= rhs.x();
        *self.y_mut() *= rhs.y();
        *self.z_mut() *= rhs.z();
    }
}

impl Div for V3 {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl Div<f64> for V3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let rhs = V3::new(rhs, rhs, rhs);
        self / rhs
    }
}

impl DivAssign for V3 {
    fn div_assign(&mut self, rhs: Self) {
        *self.x_mut() /= rhs.x();
        *self.y_mut() /= rhs.y();
        *self.z_mut() /= rhs.z();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eq() {
        let v0 = V3::new(1.0, 2.0, 3.0);
        let v1 = v0;
        assert_eq!(v0, v1);
        let v1 = V3::new(1.0, 2.0, 3.0);
        assert_eq!(v0, v1);
        assert_ne!(v0, V3::default());
    }

    #[test]
    fn basic() {
        let v3 = V3::new(1.0, 2.0, 3.0);
        let x_0 = v3.x();
        let x_1 = v3.x();
        assert!(x_0 == x_1 && x_1 == 1.0 && v3.y() == 2.0 && v3.z() == 3.0);
        let mut v3 = V3::new(0.0, 0.0, 0.0);
        let x = v3.x_mut();
        *x = 1.0;
        assert_eq!(*x, 1.0);
        let x1 = v3.x_mut();
        *x1 = 2.0;
        assert_eq!(*x1, 2.0);
        *v3.y_mut() = 2.0;
        assert_eq!(v3.y(), 2.0);
        *v3.z_mut() = 2.0;
        assert_eq!(v3.z(), 2.0);
        assert_eq!(v3.length_squared(), 2.0 * 2.0 + 2.0 * 2.0 + 2.0 * 2.0);
        assert_eq!(v3.length(), f64::sqrt(2.0 * 2.0 + 2.0 * 2.0 + 2.0 * 2.0));
    }

    #[test]
    fn index() {
        let v3 = V3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, v3[0]);
        assert_eq!(2.0, v3[1]);
        assert_eq!(3.0, v3[2]);
    }

    #[test]
    fn index_mut() {
        let mut v3 = V3::new(1.0, 2.0, 3.0);
        v3[0] = 0.0;
        v3[1] = 0.0;
        v3[2] = 0.0;
        assert_eq!(0.0, v3[0]);
        assert_eq!(0.0, v3[1]);
        assert_eq!(0.0, v3[2]);
    }

    #[test]
    fn neg() {
        let v3 = V3::new(0.0, 1.0, -1.0);
        let v3 = -v3;
        assert_eq!(0.0, v3.x());
        assert_eq!(-1.0, v3.y());
        assert_eq!(1.0, v3.z());
    }

    #[test]
    fn add() {
        let mut v0 = V3::new(0.0, 1.0, -1.0);
        let v1 = V3::new(1.0, 1.0, 1.0);
        v0 += v1;
        assert_eq!(v0.x(), 0.0 + 1.0);
        assert_eq!(v0.y(), 1.0 + 1.0);
        assert_eq!(v0.z(), -1.0 + 1.0);
        let rhs = V3::new(2.0, 2.0, 2.0);
        let v2 = v0 + rhs;
        let v3 = v0 + 2.0;
        v0 += rhs;
        assert_eq!(v0.x(), 0.0 + 1.0 + 2.0);
        assert_eq!(v0.y(), 1.0 + 1.0 + 2.0);
        assert_eq!(v0.z(), -1.0 + 1.0 + 2.0);
        assert_eq!(v0, v2);
        assert_eq!(v0, v3);
    }

    #[test]
    fn sub() {
        let mut v0 = V3::new(0.0, 1.0, -1.0);
        let v1 = V3::new(1.0, 1.0, 1.0);
        v0 -= v1;
        assert_eq!(v0.x(), 0.0 - 1.0);
        assert_eq!(v0.y(), 1.0 - 1.0);
        assert_eq!(v0.z(), -1.0 - 1.0);
        let rhs = V3::new(2.0, 2.0, 2.0);
        let v2 = v0 - rhs;
        let v3 = v0 - 2.0;
        v0 -= rhs;
        assert_eq!(v0.x(), 0.0 - 1.0 - 2.0);
        assert_eq!(v0.y(), 1.0 - 1.0 - 2.0);
        assert_eq!(v0.z(), -1.0 - 1.0 - 2.0);
        assert_eq!(v0, v2);
        assert_eq!(v0, v3);
    }

    #[test]
    fn mul() {
        let mut v0 = V3::new(0.0, 1.0, -1.0);
        let v1 = V3::new(1.0, 1.0, 1.0);
        v0 *= v1;
        assert_eq!(v0.x(), 0.0 * 1.0);
        assert_eq!(v0.y(), 1.0 * 1.0);
        assert_eq!(v0.z(), -1.0 * 1.0);
        let rhs = V3::new(2.0, 2.0, 2.0);
        let v2 = v0 * rhs;
        let v3 = v0 * 2.0;
        v0 *= rhs;
        assert_eq!(v0.x(), 0.0 * 1.0 * 2.0);
        assert_eq!(v0.y(), 1.0 * 1.0 * 2.0);
        assert_eq!(v0.z(), -1.0 * 1.0 * 2.0);
        assert_eq!(v0, v2);
        assert_eq!(v0, v3);
    }

    #[test]
    fn div() {
        let mut v0 = V3::new(0.0, 1.0, -1.0);
        let v1 = V3::new(1.0, 1.0, 1.0);
        v0 /= v1;
        assert_eq!(v0.x(), 0.0 / 1.0);
        assert_eq!(v0.y(), 1.0 / 1.0);
        assert_eq!(v0.z(), -1.0 / 1.0);
        let rhs = V3::new(2.0, 2.0, 2.0);
        let v2 = v0 / rhs;
        let v3 = v0 / 2.0;
        v0 /= rhs;
        assert_eq!(v0.x(), 0.0 / 1.0 / 2.0);
        assert_eq!(v0.y(), 1.0 / 1.0 / 2.0);
        assert_eq!(v0.z(), -1.0 / 1.0 / 2.0);
        assert_eq!(v0, v2);
        assert_eq!(v0, v3);
    }
}
