use std::io::{stdout, Write, Result};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType, size as terminal_size, enable_raw_mode, disable_raw_mode},
};
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
    for row in top..bottom + 2 {
        execute!(stdout, MoveTo(left - 1, row), Print("|"))?;
        execute!(stdout, MoveTo(right, row), Print("|"))?;
    }
    execute!(
        stdout,
        MoveTo(left - 1, bottom + 2),
        Print("‾".repeat(BOARD_WIDTH as usize + 2)))?;

    stdout.flush()?;
    Ok(())
}

pub fn render_shape(shape: &Shape) -> Result<()> {
    set_color(shape.color)?;
    render_shape_no_color(shape)?;
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
