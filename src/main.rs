extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread;
use std::time;
use lib::types::{Grid, SnakeHead, Food};
use std::path::Path;
use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;

pub mod lib;

use crate::lib::snake;


fn main() {
    let canvas_width: u32 = 760;
    let canvas_height: u32 = 760;
    let cell_width: u32 = 12;
    let columns: u32 = 64_u32;
    let rows: u32 = 64_u32;

    let mut running: bool = false;

    let (canvas, events) = lib::init(canvas_width, canvas_height);
    
    let mut canvas = canvas;
    let mut events = events;
    let ttf_context = sdl2::ttf::init().unwrap();
    let texture_creator = canvas.texture_creator();

    let font_path = Path::new("/home/cnsr/rusted/snake/src/font.ttf");

    let mut font = ttf_context.load_font(font_path, 32).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut grid: Grid = lib::grid_init(columns, rows);
    
    thread::spawn(move || {
    });


    let mut direction: (i32, i32) = (1,0);

    let mut snake: SnakeHead = snake::init_snake(direction);
    let mut food: Food = snake::init_food();

    'game: loop {
        if snake.collides() {
            break 'game;
        }
        // only works once
        // probably because snake moves twice somewhere
        if snake.collides_food((food.row, food.col)) {
            println!("Collides");
            food = snake::init_food();
            snake.increment();
            grid = snake::draw_food(&mut food, grid);
        }
        // TODO: update snake post
        // TODO: change grid with snake position
        //lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if snake.direction.0 != 1 {
                        snake.direction.0 = -1;
                        snake.direction.1 = 0;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if snake.direction.0 != -1 {
                        snake.direction.0 = 1;
                        snake.direction.1 = 0;
                    }
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if snake.direction.1 != 1 {
                        snake.direction.0 = 0;
                        snake.direction.1 = -1;
                    }
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if snake.direction.1 != -1 {
                        snake.direction.0 = 0;
                        snake.direction.1 = 1;
                    }
                },
                // pause game toggle
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    running = !running;
                },
                // Handle game exit
                Event::Quit {..} | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                // handle everything else - do nothing
                _ => continue 'game,
            }
        }
        if running {
            snake.move_snake((columns as i32, rows as i32));
            grid = snake::draw_body(&mut snake, grid, direction);
            grid = snake::draw_food(&mut food, grid);
            grid = snake::change_grid(grid, &snake, (columns, rows));
        }
        lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);

        let mut debug_text = format!("x {:?} y {:?} len {:?} seg {:?} fx {:?} fy {:?}",
            snake.column, snake.row, snake.len, snake.body.len(),
            food.col, food.row,
        );

        let surface = font.render(debug_text.as_str())
            .blended(Color::RGBA(255, 128, 0, 255)).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface).unwrap();

        let target = Rect::new(0,0,200,25);
        
        canvas.copy(&texture, None, Some(target));
        // canvas.present();
        thread::sleep(time::Duration::from_millis(80));
    }
}
