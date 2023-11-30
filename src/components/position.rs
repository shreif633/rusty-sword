use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Position {
    pub fn calculate_distance<T: Coordinate>(&self, b: &T) -> u32 {
        let (bx, by) = b.get_xy();
        let x_diff = self.x as f64 - bx as f64;
        let y_diff = self.y as f64 - by as f64;
        // Euclidean distance formula: sqrt((x2 - x1)^2 + (y2 - y1)^2)
        ((x_diff.powi(2) + y_diff.powi(2)) as f64).sqrt().round() as u32
    }

    pub fn is_in_range<T: Coordinate>(&self, b: &T, range: u32) -> bool {
        self.calculate_distance(b) < range
    }

    pub fn is_in_sight<T: Coordinate>(&self, b: &T) -> bool {
        self.is_in_range(b, 900)
    }
}

impl Coordinate for &Position {
    fn get_xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

pub trait Coordinate {
    fn get_xy(&self) -> (u32, u32);
}