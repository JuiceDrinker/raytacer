use super::vec3::Vec3;
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(point: Vec3, direction: Vec3) -> Self {
        Ray {
            origin: point,
            direction,
        }
    }

    fn point(self) -> Vec3 {
        self.origin
    }

    fn direction(self) -> Vec3 {
        self.direction
    }

    fn at(self, t: f32) -> Vec3 {
        self.origin + (t * self.direction)
    }
}
