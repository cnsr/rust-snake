use crate::lib::types::{Cell, SnakeHead, Grid, SnakeSegment, Food};
use crate::lib::grid_init;
extern crate rand;
use rand::Rng;


pub fn change_grid(mut grid: Grid, head: &SnakeHead, dimensions: (u32, u32)) -> Grid{
    let color: Cell = Cell {
        red: head.color.red,
        green: head.color.green,
        blue: head.color.green,
    };

    grid.grid[head.row as usize][head.column as usize] = color;

    grid
}

pub fn init_snake(direction: (i32, i32)) -> SnakeHead {
    let mut head: SnakeHead = SnakeHead {
        row: 24,
        column: 24,
        color: Cell {red: 255_u8, green: 0_u8, blue: 0_u8},
        len: 24, // if > 26 will panic
        body: Vec::new(),
        direction: direction,
    };
    head.init();
    head
}

pub fn init_food() -> Food {
    let mut rng = rand::thread_rng();
    let mut food: Food = Food {
        score: 1,
        row: rng.gen_range(0, 60),
        col: rng.gen_range(0, 60),
        color: Cell {red: 0_u8, green: 255_u8, blue: 0_u8},
    };
    if !(food.row % 2 == 0) {
        food.row += 1;
    }
    if !(food.col % 2 == 0) {
        food.col += 1;
    }
    food
}

pub fn draw_body(head: &mut SnakeHead, mut grid: Grid, direction: (i32, i32)) -> Grid {
    grid.reset();
    let cell_color: Cell = Cell{red: 0_u8, green: 196_u8, blue: 0_u8};
    let mut prev: (i32, i32);
    for x in (0..head.len).rev() {
        if x >= 1 {
            prev = (head.body[x as usize].row, head.body[x as usize].column);
        } else {
            prev = (head.row, head.column);
        }
        grid.grid[prev.0 as usize][prev.1 as usize] = head.body[x as usize].color;
    }
    grid
}

pub fn draw_food(food: &mut Food, mut grid: Grid) -> Grid {
    food.draw(grid)
}