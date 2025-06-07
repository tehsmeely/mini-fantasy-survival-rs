use godot::builtin::Vector2;

#[derive(Debug, Clone, Copy, Default)]
pub enum Facing8 {
    Up,
    Left,
    Right,
    #[default]
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Facing8 {
    pub fn from_vector(vector: Vector2) -> Self {
        let x = vector.x;
        let y = vector.y;
        // Vector is normalised so we can use the x/y magnitude to imply 100% in that direction.
        match (x, y) {
            (0.0, -1.0) => Self::Up,
            (0.0, 1.0) => Self::Down,
            (1.0, 0.0) => Self::Right,
            (-1.0, 0.0) => Self::Left,
            (x, y) if x > 0.0 && y > 0.0 => Self::DownRight,
            (x, y) if x < 0.0 && y > 0.0 => Self::DownLeft,
            (x, y) if x < 0.0 && y < 0.0 => Self::UpLeft,
            (x, y) if x > 0.0 && y < 0.0 => Self::UpRight,
            _ => Self::default(),
        }
    }

    pub fn from_any_vector(vector: Vector2) -> Self {
        let angle = vector.y.atan2(vector.x); // angle in radians
        let angle_deg = angle.to_degrees();
        let angle_deg = (angle_deg + 360.0) % 360.0; // normalize to [0, 360)

        // Divide the circle into 8 sectors (each 45 degrees)
        match angle_deg {
            a if !(22.5..337.5).contains(&a) => Facing8::Right,
            a if (22.5..67.5).contains(&a) => Facing8::DownRight,
            a if (67.5..112.5).contains(&a) => Facing8::Down,
            a if (112.5..157.5).contains(&a) => Facing8::DownLeft,
            a if (157.5..202.5).contains(&a) => Facing8::Left,
            a if (202.5..247.5).contains(&a) => Facing8::UpLeft,
            a if (247.5..292.5).contains(&a) => Facing8::Up,
            a if (292.5..337.5).contains(&a) => Facing8::UpRight,
            _ => unreachable!(),
        }
    }

    pub fn to_rotation(&self) -> f32 {
        match self {
            Facing8::Up => 0.0,
            Facing8::Down => 180.0,
            Facing8::Left => 270.0,
            Facing8::Right => 90.0,
            Facing8::UpLeft => -45.0,
            Facing8::UpRight => 45.0,
            Facing8::DownRight => 135.0,
            Facing8::DownLeft => 225.0,
        }
    }
}
