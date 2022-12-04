use rand::Rng;

#[derive(Clone)]
pub(crate) struct Board {
    pub(crate) grid: Grid,
    pub(crate) snake: Snake,
    pub(crate) food: Option<Cell>,
}

#[derive(Clone)]
pub(crate) struct Grid {
    pub(crate) width: usize,
    pub(crate) height: usize,
}

#[derive(Clone)]
pub(crate) struct Snake {
    pub(crate) body: Vec<Cell>,
    pub(crate) direction: Direction,
}

#[derive(PartialEq, Clone)]
pub(crate) struct Cell {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[derive(Clone)]
pub(crate) struct Direction {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

pub(crate) fn make_board(grid: Grid) -> Board {
    let snake = Snake {
        body: vec![Cell {
            x: grid.width / 2,
            y: grid.height / 2,
        }],
        direction: Direction { x: 1, y: 0 },
    };

    let food = make_food(&grid, &snake);

    Board { grid, snake, food }
}

pub(crate) fn step_board(board: &Board, direction: Direction) -> (Option<Board>, &str) {
    let mut board = board.clone();

    let old_direction = &board.snake.direction;
    if !(old_direction.x + direction.x == 0 && old_direction.y + direction.y == 0) {
        board.snake.direction = direction;
    }

    let new_head = next_head(&board);

    if board.food.as_ref() == Some(&new_head) {
        board.snake.body.insert(0, new_head);

        board.food = make_food(&board.grid, &board.snake);

        (Some(board), "eat")
    } else {
        board.snake.body.pop();

        if board.snake.body.contains(&new_head) { (None, "crash") } else {
            board.snake.body.insert(0, new_head);

            (Some(board), "")
        }
    }
}

pub(crate) fn is_board_full(board: &Board) -> bool {
    is_snake_full(&board.grid, &board.snake)
}

fn next_head(board: &Board) -> Cell {
    let head = &board.snake.body[0];
    let direction = &board.snake.direction;
    let grid = &board.grid;

    Cell {
        x: (head.x as isize + direction.x).rem_euclid(grid.width as isize) as usize,
        y: (head.y as isize + direction.y).rem_euclid(grid.height as isize) as usize,
    }
}

fn make_food(grid: &Grid, snake: &Snake) -> Option<Cell> {
    if is_snake_full(grid, snake) { None } else {
        let make = || { random_cell(&grid) };

        let mut food = make();
        while snake.body.contains(&food) { food = make(); }

        Some(food)
    }
}

fn is_snake_full(grid: &Grid, snake: &Snake) -> bool {
    snake.body.len() == grid.width * grid.height
}

fn random_cell(grid: &Grid) -> Cell {
    let mut rng = rand::thread_rng();

    Cell {
        x: rng.gen_range(0..grid.width),
        y: rng.gen_range(0..grid.height),
    }
}
