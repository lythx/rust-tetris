use std::cmp::{max, min};
use crossterm::style::Color;
use crate::shape::{Shape, ShapeType};

const BOARD_WIDTH_IN_TILES: u16 = 10;
const BOARD_HEIGHT_IN_TILES: u16 = 20;

pub fn create_shape_and_check_collision(others: &Vec<Shape>) -> (Shape, bool) {
    let shape = Shape::create_shape(ShapeType::L, 5, 0, Color::Red);
    let is_colliding = 
        check_collision_with_walls(&shape) || check_collision_with_shapes(&shape, others);
    (shape, is_colliding)   
}

pub fn fall_instantly(shape: &mut Shape, others: &Vec<Shape>) {
    while !check_collision_with_shapes(shape, others) && !check_collision_with_walls(shape) {
        shape.y += 1;
    }
}

pub fn try_move_left(shape: &mut Shape, others: &Vec<Shape>) -> bool {
    try_move(shape, others, -1, 0)
}

pub fn try_move_right(shape: &mut Shape, others: &Vec<Shape>) -> bool {
    try_move(shape, others, 1, 0)
}

pub fn try_fall(shape: &mut Shape, others: &Vec<Shape>) -> bool {
    try_move(shape, others, 0, 1)   
}

pub fn try_rotate(shape: &mut Shape, others: &Vec<Shape>) -> bool {
    shape.rotate(1);
    if check_collision_with_shapes(shape, others) || check_collision_with_walls(shape) {
        shape.rotate(-1);
        return false;
    }
    true
}

fn try_move(shape: &mut Shape, others: &Vec<Shape>, dx: i16, dy: i16) -> bool {
    if shape.x + dx < 0 || shape.y + dy < 0 {
        return false
    }
    
    shape.x = shape.x + dx;
    shape.y = shape.y + dy;
    if check_collision_with_shapes(shape, others) || check_collision_with_walls(shape) {
        shape.x = shape.x - dx;
        shape.y = shape.y - dy;
        return false
    }
    true
}

fn check_collision_with_shapes(shape: &Shape, others: &Vec<Shape>) -> bool {
    for other in others {
        if shape.check_collision(other) {
            return true;
        }
    }
    false
}

fn check_collision_with_walls(shape: &Shape) -> bool {
    let mut top: i16 = 3;
    let mut right: i16 = 0;
    let mut bottom: i16 = 0;
    let mut left: i16 = 3;
    let mat = shape.get_matrix();
    for x in 0..4 {
        for y in 0..4 {
            if mat[x][y] == 1 {
                top = min(top, y as i16);
                right = max(right, x as i16);
                bottom = max(bottom, y as i16);
                left = min(left, x as i16);
            }
        }
    }
    
    shape.x + left < 0 || shape.y + top < 0 || 
        shape.x + right > BOARD_WIDTH_IN_TILES as i16 || 
        shape.y + bottom > BOARD_HEIGHT_IN_TILES as i16
}

 


