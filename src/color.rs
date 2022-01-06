use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    v: crate::v3::V3,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            v: crate::v3::V3::new(r, g, b),
        }
    }

    pub fn r(&self) -> f64 {
        self.v[0]
    }

    pub fn r_mut(&mut self) -> &mut f64 {
        &mut self.v[0]
    }

    pub fn g(&self) -> f64 {
        self.v[1]
    }

    pub fn g_mut(&mut self) -> &mut f64 {
        &mut self.v[1]
    }

    pub fn b(&self) -> f64 {
        self.v[2]
    }

    pub fn b_mut(&mut self) -> &mut f64 {
        &mut self.v[2]
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.v += rhs.v
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.v -= rhs.v
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.v *= rhs.v
    }
}

impl DivAssign for Color {
    fn div_assign(&mut self, rhs: Self) {
        self.v /= rhs.v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let c = Color::new(1.0, 2.0, 3.0);
        let r_0 = c.r();
        let r_1 = c.r();
        assert!(r_0 == r_1 && r_1 == 1.0 && c.g() == 2.0 && c.b() == 3.0);
        let mut c = Color::new(0.0, 0.0, 0.0);
        let r = c.r_mut();
        *r = 1.0;
        assert_eq!(*r, 1.0);
        let r1 = c.r_mut();
        *r1 = 2.0;
        assert_eq!(*r1, 2.0);
        *c.g_mut() = 2.0;
        assert_eq!(c.g(), 2.0);
        *c.b_mut() = 2.0;
        assert_eq!(c.b(), 2.0);
    }

    #[test]
    fn add() {
        let mut c0 = Color::new(0.0, 1.0, -1.0);
        let c1 = Color::new(1.0, 1.0, 1.0);
        c0 += c1;
        assert_eq!(c0.r(), 0.0 + 1.0);
        assert_eq!(c0.g(), 1.0 + 1.0);
        assert_eq!(c0.b(), -1.0 + 1.0);
        let c1 = Color::new(2.0, 2.0, 2.0);
        c0 += c1;
        assert_eq!(c0.r(), 0.0 + 1.0 + 2.0);
        assert_eq!(c0.g(), 1.0 + 1.0 + 2.0);
        assert_eq!(c0.b(), -1.0 + 1.0 + 2.0);
    }

    #[test]
    fn sub() {
        let mut c0 = Color::new(0.0, 1.0, -1.0);
        let c1 = Color::new(1.0, 1.0, 1.0);
        c0 -= c1;
        assert_eq!(c0.r(), 0.0 - 1.0);
        assert_eq!(c0.g(), 1.0 - 1.0);
        assert_eq!(c0.b(), -1.0 - 1.0);
        let c1 = Color::new(2.0, 2.0, 2.0);
        c0 -= c1;
        assert_eq!(c0.r(), 0.0 - 1.0 - 2.0);
        assert_eq!(c0.g(), 1.0 - 1.0 - 2.0);
        assert_eq!(c0.b(), -1.0 - 1.0 - 2.0);
    }

    #[test]
    fn mul() {
        let mut c0 = Color::new(0.0, 1.0, -1.0);
        let c1 = Color::new(1.0, 1.0, 1.0);
        c0 *= c1;
        assert_eq!(c0.r(), 0.0 * 1.0);
        assert_eq!(c0.g(), 1.0 * 1.0);
        assert_eq!(c0.b(), -1.0 * 1.0);
        let c1 = Color::new(2.0, 2.0, 2.0);
        c0 *= c1;
        assert_eq!(c0.r(), 0.0 * 1.0 * 2.0);
        assert_eq!(c0.g(), 1.0 * 1.0 * 2.0);
        assert_eq!(c0.b(), -1.0 * 1.0 * 2.0);
    }

    #[test]
    fn div() {
        let mut c0 = Color::new(0.0, 1.0, -1.0);
        let c1 = Color::new(1.0, 1.0, 1.0);
        c0 /= c1;
        assert_eq!(c0.r(), 0.0 / 1.0);
        assert_eq!(c0.g(), 1.0 / 1.0);
        assert_eq!(c0.b(), -1.0 / 1.0);
        let c1 = Color::new(2.0, 2.0, 2.0);
        c0 /= c1;
        assert_eq!(c0.r(), 0.0 / 1.0 / 2.0);
        assert_eq!(c0.g(), 1.0 / 1.0 / 2.0);
        assert_eq!(c0.b(), -1.0 / 1.0 / 2.0);
    }
}
