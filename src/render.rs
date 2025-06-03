use std::io::{stdout, Write, Result};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType, size as terminal_size, enable_raw_mode, disable_raw_mode},
};
use crate::{LockedSquareMatrix, NextShapes};
use crate::shape::Shape;

const BOARD_WIDTH_IN_TILES: u16 = 10;
const BOARD_HEIGHT_IN_TILES: u16 = 20;
const TILE_WIDTH: u16 = 4;
const TILE_HEIGHT: u16 = 2;
const BOARD_WIDTH: u16 = BOARD_WIDTH_IN_TILES * TILE_WIDTH;
const BOARD_HEIGHT: u16 = BOARD_HEIGHT_IN_TILES * TILE_HEIGHT;

pub fn start() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All), Hide)?;
    Ok(())
}

pub fn stop() -> Result<()> {
    execute!(stdout(), ResetColor, Show)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn clear_shape(shape: &Shape) -> Result<()> {
    set_color(Color::Black)?;
    render_shape_no_color(shape)?;
    Ok(())
}

pub fn render_borders() -> Result<()> {
    let (top, right, bottom, left) = get_board_bounds();

    let mut stdout = stdout();
    set_color(Color::White)?;
    execute!(
        stdout,
        MoveTo(left - 1, top - 1),
        Print("_".repeat(BOARD_WIDTH as usize + 2))
    )?;
    for row in top..bottom {
        execute!(stdout, MoveTo(left - 1, row), Print("|"))?;
        execute!(stdout, MoveTo(right, row), Print("|"))?;
    }
    execute!(
        stdout,
        MoveTo(left - 1, bottom),
        Print("‾".repeat(BOARD_WIDTH as usize + 2)))?;

    stdout.flush()?;
    Ok(())
}

pub fn render_shape(shape: &Shape) -> Result<()> {
    set_color(shape.color)?;
    render_shape_no_color(shape)?;
    Ok(())
}

pub fn render_locked_squares(locked_squares: &LockedSquareMatrix) -> Result<()> {
    for x in 0..locked_squares.len() {
        for y in 0..locked_squares[x].len() {
            if locked_squares[x][y].is_some() {
                set_color(locked_squares[x][y].unwrap())?;
                render_square(x as u16, y as u16)?;
            }
        }
    }
    Ok(())
}

pub fn clear_locked_squares(locked_squares: &LockedSquareMatrix) -> Result<()> {
    set_color(Color::Black)?;
    for x in 0..locked_squares.len() {
        for y in 0..locked_squares[x].len() {
            if locked_squares[x][y].is_some() {
                render_square(x as u16, y as u16)?;
            }
        }
    }
    Ok(())
}

pub fn render_next_shapes_borders() -> Result<()> {
    let (top, right, bottom, left) = get_next_shapes_bounds();
    let mut stdout = stdout();
    set_color(Color::White)?;
    execute!(
        stdout,
        MoveTo(left - 1, top - 1),
        Print("_".repeat((right - left + 2) as usize))
    )?;
    for row in top..bottom {
        execute!(stdout, MoveTo(left - 1, row), Print("|"))?;
        execute!(stdout, MoveTo(right, row), Print("|"))?;
    }
    execute!(
        stdout,
        MoveTo(left - 1, bottom),
        Print("‾".repeat((right - left + 2) as usize))
    )?;
    stdout.flush()?;
    Ok(())
}

pub fn clear_next_shapes(next_shapes: &mut NextShapes) -> Result<()> {
    let x = BOARD_WIDTH_IN_TILES + 1;
    let mut y = 1;

    for s in next_shapes {
        s.x = x as i16;
        s.y = y as i16;
        clear_shape(s)?;
        y += 5;
    }

    Ok(())
}

pub fn render_next_shapes(next_shapes: &mut NextShapes) -> Result<()> {
    let x = BOARD_WIDTH_IN_TILES + 1;
    let mut y = 1;

    for s in next_shapes {
        s.x = x as i16;
        s.y = y as i16;
        render_shape(s)?;
        y += 5;
    }

    Ok(())
}

pub fn render_score(score: u32) -> Result<()> {
    set_color(Color::Green)?;
    let (top, right, _, _) = get_board_bounds();
    let x = right + 3;
    let y = top + 31;
    let mut stdout = stdout();
    execute!(
        stdout,
        MoveTo(x, y),
        Print("Score: "),
        Print(score.to_string())
    )?;
    Ok(())
}

fn render_shape_no_color(shape: &Shape) -> Result<()> {
    for dx in 0..4 {
        for dy in 0..4 {
            if shape.get_matrix()[dx][dy] == 1 {
                render_square((shape.x + dx as i16) as u16, (shape.y + dy as i16) as u16)?;
            }
        }
    }
    Ok(())
}

fn get_board_bounds() -> (u16, u16, u16, u16) {
    let (terminal_width, terminal_height) = terminal_size().unwrap();
    let bottom = terminal_height - 3;
    let top = bottom - BOARD_HEIGHT;
    let left = terminal_width / 2 - BOARD_WIDTH / 2;
    let right = left + BOARD_WIDTH;
    (top, right, bottom, left)
}

fn get_next_shapes_bounds() -> (u16, u16, u16, u16) {
    let (board_top, board_right, _, _) = get_board_bounds();
    let top = board_top;
    let bottom = top + 30;
    let left = board_right + 3;
    let right = left + 10;
    (top, right, bottom, left)
}

fn render_square(x: u16, y: u16) -> Result<()> {
    let (top, _, _, left) = get_board_bounds();
    let board_x = x * TILE_WIDTH + left;
    let board_y = y * TILE_HEIGHT + top;
    let mut stdout = stdout();
    execute!(
        stdout,
        MoveTo(board_x, board_y),
        Print("░░░░"))?;
    execute!(
        stdout,
        MoveTo(board_x, board_y + 1),
        Print("░░░░"))?;
    Ok(())
}

fn set_color(color: Color) -> Result<()> {
    execute!(stdout(), SetForegroundColor(color))
}
