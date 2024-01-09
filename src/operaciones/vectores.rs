// debe ser todo publico para poder usarse
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
}

pub fn suma(v1: Vector2, v2: Vector2) -> Vector2 {
    Vector2 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
    }
}
