#![allow(dead_code)]

#[derive(Copy, Clone)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}


macro_rules! maxf {
    ($x: expr, $z: expr) => {
        if $x > $z {
            $x
        } else {
            $z
        }
    }
}
#[macro_export]
macro_rules! minf {
    ($x: expr, $z: expr) => {
        if $x < $z {
            $x
        } else {
            $z
        }
    }
}


pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn from(x: f32) -> Self {
        Self { x, y: x }
    }

    #[inline]
    pub fn u(&self) -> f32 { self.x }
    #[inline]
    pub fn v(&self) -> f32 { self.y }

    pub fn sum(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
    pub fn sub(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
    pub fn mul(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
    pub fn div(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
    pub fn max(&mut self, other: Self) {
        self.x = maxf!(self.x, other.x);
        self.y = maxf!(self.y, other.y);
    }
    pub fn min(&mut self, other: Self) {
        self.x = minf!(self.x, other.x);
        self.y = minf!(self.y, other.y);
    }
    pub fn sqr_len(&self) -> f32 {
        return self.x * self.x + self.y * self.y;
    }
    pub fn lerp(&mut self, b: &Self, t: &Self) {
        self.x = lerp(self.x, b.x, t.x);
        self.y = lerp(self.y, b.y, t.y);
    }
    pub fn sqrt(&mut self) {
        self.x = self.x.sqrt();
        self.y = self.y.sqrt();
    }
    pub fn pow(&mut self, other: Self) {
        self.x = self.x.powf(other.x);
        self.y = self.y.powf(other.y);
    }
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub const fn new_c(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn from(x: f32) -> Self {
        Self { x, y: x, z: x }
    }

    #[inline]
    pub fn r(&self) -> f32 { self.x }
    #[inline]
    pub fn g(&self) -> f32 { self.y }
    #[inline]
    pub fn b(&self) -> f32 { self.z }

    pub fn sum(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    pub fn mul(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
    pub fn div(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
    pub fn max(&mut self, other: Self) {
        self.x = maxf!(self.x, other.x);
        self.y = maxf!(self.y, other.y);
        self.z = maxf!(self.z, other.z);
    }
    pub fn min(&mut self, other: Self) {
        self.x = minf!(self.x, other.x);
        self.y = minf!(self.y, other.y);
        self.z = minf!(self.z, other.z);
    }
    pub fn sqr_len(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn lerp(&mut self, b: &Self, t: &Self) {
        self.x = lerp(self.x, b.x, t.x);
        self.y = lerp(self.y, b.y, t.y);
        self.z = lerp(self.z, b.z, t.z);
    }
    pub fn lerp_l(&mut self, b: &Self, t: &Self) {
        self.x = lerp(t.x, b.x, self.x);
        self.y = lerp(t.y, b.y, self.y);
        self.z = lerp(t.z, b.z, self.z);
    }
    pub fn sqrt(&mut self) {
        self.x = self.x.sqrt();
        self.y = self.y.sqrt();
        self.z = self.z.sqrt();
    }
    pub fn pow(&mut self, other: Self) {
        self.x = self.x.powf(other.x);
        self.y = self.y.powf(other.y);
        self.z = self.z.powf(other.z);
    }
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub fn from(x: f32) -> Self {
        Self { x, y: x, z: x, w: x }
    }

    #[inline]
    pub fn r(&self) -> f32 { self.x }
    #[inline]
    pub fn g(&self) -> f32 { self.y }
    #[inline]
    pub fn b(&self) -> f32 { self.z }
    #[inline]
    pub fn a(&self) -> f32 { self.w }

    pub fn sum(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
    pub fn mul(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
    pub fn div(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
    pub fn max(&mut self, other: Self) {
        self.x = maxf!(self.x, other.x);
        self.y = maxf!(self.y, other.y);
        self.z = maxf!(self.z, other.z);
        self.w = maxf!(self.w, other.w);
    }
    pub fn min(&mut self, other: Self) {
        self.x = minf!(self.x, other.x);
        self.y = minf!(self.y, other.y);
        self.z = minf!(self.z, other.z);
        self.w = minf!(self.w, other.w);
    }
    pub fn sqr_len(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    }
    pub fn lerp(&mut self, b: Self, t: Self) {
        self.x = lerp(self.x, b.x, t.x);
        self.y = lerp(self.y, b.y, t.y);
        self.z = lerp(self.z, b.z, t.z);
        self.w = lerp(self.w, b.w, t.w);
    }
    pub fn sqrt(&mut self) {
        self.x = self.x.sqrt();
        self.y = self.y.sqrt();
        self.z = self.z.sqrt();
        self.w = self.w.sqrt();
    }
    pub fn pow(&mut self, other: Self) {
        self.x = self.x.powf(other.x);
        self.y = self.y.powf(other.y);
        self.z = self.z.powf(other.z);
        self.w = self.w.powf(other.w);
    }
}


