mod render;
mod shape;
mod input;
mod update;

use std::io::{Result};
use crossterm::style::Color;
use crate::render::render_shape;
use crate::shape::{Shape, ShapeType};
use crate::update::{create_shape_and_check_collision, try_fall};

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

    let mut i = 0;

    loop {
        // update
        let prev_falling_shape = falling_shape.clone();
        if !try_fall(&mut falling_shape, &locked_shapes) {
            locked_shapes.push(falling_shape.clone());
            let (new_falling_shape, is_colliding) =
                create_shape_and_check_collision(&locked_shapes);
            if is_colliding {
                break;
            }
            falling_shape = new_falling_shape;
        }

        render::clear_shape(&prev_falling_shape)?;

        render::render_shape(&falling_shape)?;
        for locked_shape in &locked_shapes {
            render::render_shape(locked_shape)?;
        }

        i += 1;
        if i == 2000 {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    render::stop()?;
    Ok(())
}
