mod render;
mod shape;
mod input;
mod update;

use std::io::{Result};
use std::time::{Duration, SystemTime};
use crossterm::style::Color;
use crate::input::{receive_input, Action};
use crate::update::{create_shape_and_check_collision, fall_instantly, try_fall, try_move_left, try_move_right, try_rotate};

pub const BOARD_WIDTH_IN_TILES: usize = 10;
pub const BOARD_HEIGHT_IN_TILES: usize = 20;

pub type LockedSquareMatrix = [[Option<Color>; BOARD_HEIGHT_IN_TILES]; BOARD_WIDTH_IN_TILES];

fn main() -> Result<()> {
    render::start()?;
    render::render_borders()?;

    // let s1 = Shape::create_shape(ShapeType::L, 1, 4, Color::Red);
    // let s2 = Shape::create_shape(ShapeType::L, 1, 2, Color::Blue);
    //
    // render_shape(&s1)?;
    // render_shape(&s2)?;
    // println!("{}", s1.check_collision(&s2));


    let mut locked_squares: LockedSquareMatrix = [[None; BOARD_HEIGHT_IN_TILES]; BOARD_WIDTH_IN_TILES];
    let mut falling_shape = create_shape_and_check_collision(&locked_squares).0;
    let mut next_fall = SystemTime::now() + Duration::from_millis(1000);

    loop {
        // update
        let prev_falling_shape = falling_shape.clone();
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

        if finished_falling {
            for (x, y) in falling_shape.get_occupied_squares() {
                locked_squares[x as usize][y as usize] = Some(falling_shape.color);
            }
            let (new_falling_shape, is_colliding) =
                create_shape_and_check_collision(&locked_squares);
            if is_colliding {
                break;
            }
            falling_shape = new_falling_shape;
            let old_locked_squares = locked_squares.clone();
            if update::delete_full_rows(&mut locked_squares) != 0 {
         
            }
            render::clear_locked_squares(&old_locked_squares)?; // todo this only on 0
            render::render_locked_squares(&locked_squares)?; // todo this only on 0
        }

        if prev_falling_shape != falling_shape {
            render::clear_shape(&prev_falling_shape)?;
        }

        render::render_shape(&falling_shape)?;

        std::thread::sleep(Duration::from_millis(50));
    }

    render::stop()?;
    Ok(())
}
