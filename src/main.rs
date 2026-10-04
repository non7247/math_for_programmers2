use std::ops;

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, v2: Vec2) -> Self::Output {
        Vec2 { x: self.x + v2.x, y: self.y + v2.y }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec2 { x: self.x * scalar, y: self.y * scalar }
    }
}

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, v2: Vec3) -> Self::Output {
        Vec3 { x: self.x + v2.x, y: self.y + v2.y, z: self.z + v2.z }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

fn average<T>(v1: &T, v2: &T) -> T
where
    T: Copy + ops::Add<Output = T> + ops::Mul<f64, Output = T>,
{
    *v1 * 0.5 + *v2 * 0.5
}

fn main() {
    let v = Vec2 { x: 1.0, y: 0.0 } * 3.0 + Vec2 { x: 0.0, y: 1.0 } * 4.0;
    println!("{:?}", v);

    let v = (Vec3 { x: 1.0, y: 0.0, z: 0.0 } + Vec3 { x: 0.0, y: 1.0, z: 0.0 }) * 2.0;
    println!("{:?}", v);

    println!("{:?}", average(&Vec2 { x: 9.0, y: 1.0 }, &Vec2 { x: 8.0, y: 6.0 }));
}