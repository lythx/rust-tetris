use std::cmp::{max, min};
use crate::{LockedSquareMatrix, NextShapes};
use crate::shape::{Shape, ShapeType};

const BOARD_WIDTH_IN_TILES: u16 = 10;
const BOARD_HEIGHT_IN_TILES: u16 = 20;

pub fn put_next_shape_on_board_and_check_collision
        (next_shapes: &mut NextShapes, locked_squares: &LockedSquareMatrix) -> (Shape, bool) {
    let shape = Shape::new(next_shapes[0].shape_type, BOARD_WIDTH_IN_TILES as i16 / 2, 0);
    next_shapes[0] = next_shapes[1];
    next_shapes[1] = next_shapes[2];
    next_shapes[2] = Shape::new_random(0, 0);
    let is_colliding = 
        check_collision_with_walls(&shape) || check_collision_with_locked_squares(&shape, locked_squares);
    (shape, is_colliding)   
}

pub fn fall_instantly(shape: &mut Shape, locked_squares: &LockedSquareMatrix) {
    while !check_collision_with_walls(shape) && !check_collision_with_locked_squares(shape, locked_squares) {
        shape.y += 1;
    }
    shape.y -= 1;
}

pub fn try_move_left(shape: &mut Shape, locked_squares: &LockedSquareMatrix) -> bool {
    try_move(shape, locked_squares, -1, 0)
}

pub fn try_move_right(shape: &mut Shape, locked_squares: &LockedSquareMatrix) -> bool {
    try_move(shape, locked_squares, 1, 0)
}

pub fn try_fall(shape: &mut Shape, locked_squares: &LockedSquareMatrix) -> bool {
    try_move(shape, locked_squares, 0, 1)
}

pub fn try_rotate(shape: &mut Shape, locked_squares: &LockedSquareMatrix) -> bool {
    shape.rotate(1);
    if check_collision_with_locked_squares(shape, locked_squares) || check_collision_with_walls(shape) {
        shape.rotate(-1);
        return false;
    }
    true
}

pub fn delete_full_rows(locked_squares: &mut LockedSquareMatrix) -> u8 {
    let mut full_rows = Vec::new();
    for y in 0..BOARD_HEIGHT_IN_TILES as usize {
        let mut is_full_row = true;
        for x in 0..BOARD_WIDTH_IN_TILES as usize {
            if locked_squares[x][y].is_none() {
                is_full_row = false;
            }
        }
        if is_full_row {
            full_rows.push(y);
        }
    }
    if full_rows.is_empty() {
        return 0;
    }
    let mut copy_to_y = BOARD_HEIGHT_IN_TILES as usize - 1;
    for y in (0..BOARD_HEIGHT_IN_TILES as usize).rev() {
        if full_rows.contains(&y) {
            continue;
        }
        if y != copy_to_y {
            for x in 0..BOARD_WIDTH_IN_TILES as usize {
                locked_squares[x][copy_to_y] = locked_squares[x][y];
                locked_squares[x][y] = None;
            }
        }
        copy_to_y -= 1;
    }
    full_rows.len() as u8  
}

pub fn calculate_score(current_score: u32, did_shape_fall: bool, rows_deleted: u8) -> u32 {
    let mut result = current_score;
    if did_shape_fall {
        result += 25;
    }
    result + rows_deleted as u32 * 100
}

fn try_move(shape: &mut Shape, locked_squares: &LockedSquareMatrix, dx: i16, dy: i16) -> bool {
    shape.x = shape.x + dx;
    shape.y = shape.y + dy;
    if check_collision_with_locked_squares(shape, locked_squares) || check_collision_with_walls(shape) {
        shape.x = shape.x - dx;
        shape.y = shape.y - dy;
        return false
    }
    true
}

fn check_collision_with_locked_squares(shape: &Shape, locked_squares: &LockedSquareMatrix) -> bool {
    for x in 0..locked_squares.len() {
        for y in 0..locked_squares[x].len() {
            if locked_squares[x][y].is_some() && shape.is_occupying(x as i16, y as i16) {
                return true;
            }
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
        shape.x + right >= BOARD_WIDTH_IN_TILES as i16 ||
        shape.y + bottom >= BOARD_HEIGHT_IN_TILES as i16
}
