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

fn main() {
    let v = Vec2 { x: 1.0, y: 0.0 } * 3.0 + Vec2 { x: 0.0, y: 1.0 } * 4.0;
    println!("{:?}", v);
}