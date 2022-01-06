use crate::color::Color;
use std::cmp::{Eq, PartialEq};
use std::slice::{Chunks, ChunksMut};

#[derive(Debug, Default, Clone, Copy)]
pub struct P3 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl P3 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_color(c: Color) -> Self {
        Self {
            r: Self::f64_to_p(c.r()),
            g: Self::f64_to_p(c.g()),
            b: Self::f64_to_p(c.b()),
        }
    }

    fn f64_to_p(f: f64) -> u8 {
        (f.clamp(0.0, 1.0) * 255.999) as u8
    }

    pub fn output(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}

impl PartialEq for P3 {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Eq for P3 {}

#[derive(Debug, Default, Clone)]
pub struct Ppm {
    data: Vec<P3>,
    width: usize,
    height: usize,
}

impl Ppm {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![P3::default(); width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn rows(&self) -> Chunks<P3> {
        self.data.chunks(self.width)
    }

    pub fn rows_mut(&mut self) -> ChunksMut<P3> {
        self.data.chunks_mut(self.width)
    }

    pub fn output(&self) -> String {
        let mut out = format!("P3\n{} {}\n255\n", self.width, self.height);
        for p3 in &self.data {
            out = format!("{}{}\n", out, p3.output());
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p3_output() {
        let p3 = P3::new(255, 0, 122);
        assert_eq!(p3.r, 255);
        assert_eq!(p3.g, 0);
        assert_eq!(p3.b, 122);
        assert_eq!(p3.output(), "255 0 122");
        let p3_1 = P3::new(255, 0, 122);
        assert_eq!(p3, p3_1);
        let p3_1 = P3::new(0, 0, 0);
        assert_ne!(p3, p3_1);
    }

    #[test]
    fn ppm() {
        let mut ppm = Ppm::new(3, 2);
        assert_eq!(ppm.width(), 3);
        assert_eq!(ppm.height(), 2);
        assert_eq!(ppm.len(), 6);
        let rows = ppm.rows();
        assert_eq!(rows.len(), 2);
        for p3 in rows.flatten() {
            assert_eq!(*p3, P3::default());
        }
        assert_eq!(
            ppm.output(),
            "P3\n3 2\n255\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n"
        );
        let rows = ppm.rows_mut();
        for p3 in rows.flatten() {
            p3.r = 255;
            p3.g = 255;
            p3.b = 255;
        }
        let rows = ppm.rows();
        for p3 in rows.flatten() {
            assert_eq!(*p3, P3::new(255, 255, 255));
        }
        assert_eq!(ppm.output(), "P3\n3 2\n255\n255 255 255\n255 255 255\n255 255 255\n255 255 255\n255 255 255\n255 255 255\n");
    }
}
