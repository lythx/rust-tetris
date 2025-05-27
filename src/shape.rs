use crossterm::style::Color;
use rand::prelude::*;

pub type ShapeMatrix = [[u8; 4]; 4];

#[derive(Clone, Debug)]
#[derive(Copy)]
#[derive(PartialEq, Eq)]
pub struct Shape {
    pub x: i16,
    pub y: i16,
    pub color: Color,
    matrices: [ShapeMatrix; 4],
    current_matrix: u8
}

#[derive(Copy)]
#[derive(Clone)]
pub enum ShapeType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl ShapeType {
    
    pub fn random() -> ShapeType {
        let mut rng = thread_rng();
        let types = [ShapeType::I, ShapeType::J, ShapeType::L, 
            ShapeType::O, ShapeType::S, ShapeType::T, ShapeType::Z];
        *types.choose(&mut rng).expect("Could not choose a shape type")
    }
    
}

impl Shape {
    pub fn new(shape_type: ShapeType, x: i16, y: i16) -> Shape {
        let matrices: [ShapeMatrix; 4] = match shape_type {
            ShapeType::I => [
                [
                    [0, 0, 0, 0],
                    [1, 1, 1, 1],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 1, 0],
                    [0, 0, 1, 0],
                    [0, 0, 1, 0],
                    [0, 0, 1, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [1, 1, 1, 1],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                ],
            ],
            ShapeType::J => [
                [
                    [1, 0, 0, 0],
                    [1, 1, 1, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 1, 1, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [1, 1, 1, 0],
                    [0, 0, 1, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [1, 1, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            ShapeType::L => [
                [
                    [0, 0, 1, 0],
                    [1, 1, 1, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 1, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [1, 1, 1, 0],
                    [1, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [1, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            ShapeType::O => [
                [
                    [1, 1, 0, 0],
                    [1, 1, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [1, 1, 0, 0],
                    [1, 1, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [1, 1, 0, 0],
                    [1, 1, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [1, 1, 0, 0],
                    [1, 1, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            ShapeType::S => [
                [
                    [0, 1, 1, 0],
                    [1, 1, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 1, 0, 0],
                    [0, 1, 1, 0],
                    [0, 0, 1, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [0, 1, 1, 0],
                    [1, 1, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [1, 0, 0, 0],
                    [1, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            ShapeType::T => [
                [
                    [0, 1, 0, 0],
                    [1, 1, 1, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 1, 0, 0],
                    [0, 1, 1, 0],
                    [0, 1, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [1, 1, 1, 0],
                    [0, 1, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 1, 0, 0],
                    [1, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
            ShapeType::Z => [
                [
                    [1, 1, 0, 0],
                    [0, 1, 1, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 1, 0],
                    [0, 1, 1, 0],
                    [0, 1, 0, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 0, 0, 0],
                    [1, 1, 0, 0],
                    [0, 1, 1, 0],
                    [0, 0, 0, 0],
                ],
                [
                    [0, 1, 0, 0],
                    [1, 1, 0, 0],
                    [1, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
            ],
        };
        let color = match shape_type {
            ShapeType::I => Color::Cyan,
            ShapeType::J => Color::Blue,
            ShapeType::L => Color::AnsiValue(208),
            ShapeType::O => Color::Yellow,
            ShapeType::S => Color::Green,
            ShapeType::T => Color::Magenta,
            ShapeType::Z => Color::Red,
        };
        Shape { matrices, x, y, color, current_matrix: 0 }
    }

    pub fn rotate(&mut self, how_many_times: i8) {
        let keep_positive: i8 = if how_many_times > 0 {
            0
        } else {
            (how_many_times.abs() / 4 + 1) * 4
        };
        self.current_matrix = 
            (self.current_matrix as i8 + how_many_times + keep_positive) as u8 % 4;
    }
    
    pub fn get_matrix(&self) -> &ShapeMatrix {
        &self.matrices[self.current_matrix as usize]
    }
    
    pub fn is_occupying(&self, x: i16, y: i16) -> bool {
        if x - self.x >= 4 || y - self.y >= 4 || x - self.x < 0 || y - self.y < 0 {
            return false;
        }
        self.get_matrix()[(x - self.x) as usize][(y - self.y) as usize] == 1
    }
    
    pub fn get_occupied_squares(&self) -> Vec<(i16, i16)> {
        let mut result = Vec::new();
        for x in 0..4 {
            for y in 0..4 {
                if self.get_matrix()[x][y] == 1 {
                    result.push((x as i16 + self.x, y as i16 + self.y));   
                }
            }       
        }
        result
    }

}
