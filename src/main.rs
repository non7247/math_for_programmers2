#[derive(Debug, Clone)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn add(&self, v2: &Vec2) -> Vec2 {
        Vec2 { x: self.x + v2.x, y: self.y + v2.y }
    }
}

fn main() {
    let v = Vec2 { x: 3.0, y: 4.0 };
    let w = v.add(&Vec2 { x: -2.0, y: 6.0 });
    println!("{:?}", w);
}