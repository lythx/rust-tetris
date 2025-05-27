mod render;
mod shape;
mod input;
mod update;

use std::io::{Result};
use std::time::{Duration, SystemTime};
use crossterm::style::Color;
use crate::input::{receive_input, GameAction};
use crate::shape::{Shape, ShapeType};
use crate::update::{create_shape_and_check_collision, fall_instantly, try_fall, try_move_left, try_move_right, try_rotate};

fn main() -> Result<()> {
    render::start()?;
    render::render_borders()?;

    // let s1 = Shape::create_shape(ShapeType::L, 1, 4, Color::Red);
    // let s2 = Shape::create_shape(ShapeType::L, 1, 2, Color::Blue);
    //
    // render_shape(&s1)?;
    // render_shape(&s2)?;
    // println!("{}", s1.check_collision(&s2));


    let mut locked_shapes: Vec<Shape> = Vec::new();
    let mut falling_shape = create_shape_and_check_collision(&locked_shapes).0;
    let mut next_fall = SystemTime::now() + Duration::from_millis(1000);

    loop {
        // update
        let prev_falling_shape = falling_shape.clone();
        let finished_falling = if next_fall < SystemTime::now() {
            next_fall += Duration::from_millis(1000);
            !try_fall(&mut falling_shape, &locked_shapes)
        }
        else {
            match receive_input()? {
                GameAction::MoveLeft => {
                    try_move_left(&mut falling_shape, &locked_shapes);
                    false
                },
                GameAction::MoveRight => { 
                    try_move_right(&mut falling_shape, &locked_shapes);
                    false
                },
                GameAction::Rotate => { 
                    try_rotate(&mut falling_shape, &locked_shapes);
                    false
                },
                GameAction::SoftDrop => { 
                    try_fall(&mut falling_shape, &locked_shapes);
                    false
                },
                GameAction::HardDrop => { 
                    fall_instantly(&mut falling_shape, &locked_shapes);
                    true
                },
                GameAction::Quit => break,
                GameAction::None => false 
            }
        };

        if finished_falling {
            locked_shapes.push(falling_shape.clone());
            let (new_falling_shape, is_colliding) =
                create_shape_and_check_collision(&locked_shapes);
            if is_colliding {
                break;
            }
            falling_shape = new_falling_shape;
        }

        if prev_falling_shape != falling_shape {
            render::clear_shape(&prev_falling_shape)?;
        }

        render::render_shape(&falling_shape)?;
        for locked_shape in &locked_shapes {
            render::render_shape(locked_shape)?;
        }

        std::thread::sleep(Duration::from_millis(50));
    }

    render::stop()?;
    Ok(())
}
