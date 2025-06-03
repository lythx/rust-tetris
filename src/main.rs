mod render;
mod shape;
mod input;
mod update;

use std::io::{Result};
use std::time::{Duration, SystemTime};
use crossterm::style::Color;
use crate::input::{receive_input, Action};
use crate::shape::Shape;
use crate::update::{fall_instantly, put_next_shape_on_board_and_check_collision, try_fall, try_move_left, try_move_right, try_rotate};

pub const BOARD_WIDTH_IN_TILES: usize = 10;
pub const BOARD_HEIGHT_IN_TILES: usize = 20;

pub type LockedSquareMatrix = [[Option<Color>; BOARD_HEIGHT_IN_TILES]; BOARD_WIDTH_IN_TILES];
pub type NextShapes = [Shape; 3];

fn main() -> Result<()> {
    render::start()?;
    render::render_borders()?;
    render::render_next_shapes_borders()?;
    render::render_score(0)?;

    let mut next_shapes: NextShapes = [Shape::new_random(0, 0); 3];
    let mut locked_squares: LockedSquareMatrix = [[None; BOARD_HEIGHT_IN_TILES]; BOARD_WIDTH_IN_TILES];
    let mut falling_shape = 
        put_next_shape_on_board_and_check_collision(&mut next_shapes, &locked_squares).0;
    let mut next_fall = SystemTime::now() + Duration::from_millis(1000);
    let mut score = 0;

    render::render_next_shapes(&mut next_shapes)?;

    loop {
        let mut prev_next_shapes = next_shapes.clone();
        let shape_before_action = falling_shape.clone();
        
        let finished_falling = if next_fall < SystemTime::now() {
            next_fall += Duration::from_millis(1000);
            !try_fall(&mut falling_shape, &locked_squares)
        }
        else {
            match receive_input()? {
                Action::MoveLeft => {
                    try_move_left(&mut falling_shape, &locked_squares);
                    false
                },
                Action::MoveRight => {
                    try_move_right(&mut falling_shape, &locked_squares);
                    false
                },
                Action::Rotate => {
                    try_rotate(&mut falling_shape, &locked_squares);
                    false
                },
                Action::SoftDrop => {
                    try_fall(&mut falling_shape, &locked_squares);
                    false
                },
                Action::HardDrop => {
                    fall_instantly(&mut falling_shape, &locked_squares);
                    true
                },
                Action::Quit => break,
                Action::None => false
            }
        };

        let shape_after_action = falling_shape.clone();
        let mut locked_squares_after_action = locked_squares.clone();
        
        let mut rows_deleted = 0;

        if finished_falling {
            for (x, y) in falling_shape.get_occupied_squares() {
                locked_squares[x as usize][y as usize] = Some(falling_shape.color);
            }
            locked_squares_after_action = locked_squares.clone();
            let (new_falling_shape, is_colliding) =
                put_next_shape_on_board_and_check_collision(&mut next_shapes, &locked_squares);
            if is_colliding {
                break;
            }
            falling_shape = new_falling_shape;
            rows_deleted = update::delete_full_rows(&mut locked_squares);
        }

        score = update::calculate_score(score, finished_falling, rows_deleted);

        if shape_before_action != falling_shape {
            render::clear_shape(&shape_before_action)?;
        }

        if rows_deleted != 0 {
            render::clear_locked_squares(&locked_squares_after_action)?;
            render::render_locked_squares(&locked_squares)?;
        } 
        else if finished_falling {
            render::render_shape(&shape_after_action)?;
        }

        render::render_shape(&falling_shape)?;
        
        render::render_score(score)?;

        if finished_falling {
            render::clear_next_shapes(&mut prev_next_shapes)?;
        }
        render::render_next_shapes(&mut next_shapes)?;

        std::thread::sleep(Duration::from_millis(50));
    }

    render::stop()?;
    Ok(())
}
