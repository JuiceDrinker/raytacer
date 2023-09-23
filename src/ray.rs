struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    fn new(point: Point, direction: Vec3) -> Self {
        Ray {
            origin: point,
            direction,
        }
    }

    fn point(self) -> Point {
        self.origin
    }

    fn direction(self) -> Vec3 {
        self.direction
    }

    fn at(self, t: f32) -> Point {
        return self.origin + t * self.direction;
    }
}
