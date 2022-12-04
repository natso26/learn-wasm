use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

use crate::board::{Board, Cell, Direction, Grid, is_board_full, make_board, Snake, step_board};

#[wasm_bindgen]
pub struct Game {
    board: Board,
    score: usize,
    state: String,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(getter)]
    pub fn score(&self) -> usize { self.score }
    #[wasm_bindgen(getter)]
    pub fn state(&self) -> String { self.state.clone() }
}

#[wasm_bindgen]
pub fn make_game(width: usize, height: usize) -> Game {
    Game {
        board: make_board(Grid { width, height }),
        score: 0,
        state: "".into(),
    }
}

#[wasm_bindgen]
pub fn step_game(game: &Game, direction: &str) -> Option<Game> {
    if game.state != "" { return None; }

    let (new_board, event) =
        step_board(&game.board, parse_direction(direction, &game.board));

    Some(match event {
        "eat" => {
            let board = new_board.unwrap();
            let state = if is_board_full(&board) { "won" } else { "" };
            Game {
                board,
                score: game.score + 1,
                state: state.into(),
            }
        }
        "crash" => Game {
            board: game.board.clone(),
            score: game.score,
            state: "lost".into(),
        },
        "" => Game {
            board: new_board.unwrap(),
            score: game.score,
            state: "".into(),
        },
        _ => panic!("Unknown event"),
    })
}

#[wasm_bindgen]
pub fn game_pixels(game: &Game) -> Array {
    let pixels = Array::new();

    let snake_positions = snake_positions(&game.board.grid, &game.board.snake);

    for x in 0..game.board.grid.width {
        for y in 0..game.board.grid.height {
            let snake_position = snake_positions[y * game.board.grid.width + x];

            let entries = Array::new();
            entries.push(&Array::of2(&"x".into(), &x.into()));
            entries.push(&Array::of2(&"y".into(), &y.into()));
            entries.push(&Array::of2(&"kind".into(), &(if snake_position.is_some() {
                "snake"
            } else if game.board.food.as_ref() == Some(&Cell { x, y }) {
                "food"
            } else { "empty" }).into()));
            entries.push(&Array::of2(&"snakePosition".into(), &snake_position.into()));

            pixels.push(&Object::from_entries(&entries).ok().unwrap());
        }
    }

    pixels
}

fn parse_direction(raw_direction: &str, board: &Board) -> Direction {
    match raw_direction {
        "up" => Direction { x: 0, y: -1 },
        "down" => Direction { x: 0, y: 1 },
        "left" => Direction { x: -1, y: 0 },
        "right" => Direction { x: 1, y: 0 },
        "" => board.snake.direction.clone(),
        _ => panic!("Invalid direction"),
    }
}

fn snake_positions(grid: &Grid, snake: &Snake) -> Vec<Option<usize>> {
    let mut snake_positions = vec![None; grid.width * grid.height];

    for (i, cell) in snake.body.iter().enumerate() {
        snake_positions[cell.y * grid.width + cell.x] = Some(i);
    }

    snake_positions
}
