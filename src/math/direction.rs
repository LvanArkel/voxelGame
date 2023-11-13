use nalgebra::Vector3;

#[derive(PartialEq, Eq)]
pub enum Direction {
    Left, // Positive X
    Right, // Negative X
    Up, // Positive Y
    Down, // Negative Y
    Back, // Positive Z
    Front, // Negative Z
}

impl Direction {
    pub fn facing(&self) -> Vector3<i32> {
        match self {
            Direction::Left => Vector3::x(),
            Direction::Right => -Vector3::x(),
            Direction::Up => Vector3::y(),
            Direction::Down => -Vector3::y(),
            Direction::Back => Vector3::z(),
            Direction::Front => -Vector3::z(),
        }
    }
}