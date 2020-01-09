#[derive(Clone, Debug)]
pub struct Grid {
    pub grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn reset(&mut self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                self.grid[row][col] = Cell {
                    red: 255_u8,
                    green: 255_u8,
                    blue: 255_u8,
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
pub struct Food {
    pub score: i32,
    pub row: i32,
    pub col: i32,
    pub color: Cell,
}

impl Food {
    pub fn draw(&mut self, mut grid: Grid) -> Grid {
        grid.grid[self.row as usize][self.col as usize] = self.color;
        grid
    }
}

#[derive(Debug)]
pub struct SnakeHead {
    pub row: i32,
    pub column: i32,
    pub color: Cell,
    pub len: i32,
    pub direction: (i32, i32),
    pub body: Vec<SnakeSegment>,
}

#[derive(Debug)]
pub struct SnakeSegment {
    pub row: i32,
    pub column: i32,
    pub color: Cell,
}

pub fn clone_vec<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut newvec = vec.to_vec();
    newvec
}

impl SnakeHead {
    pub fn move_snake(&mut self, dimensions: (i32, i32)) {
        // move snake according to direction
        self.row = self.row + self.direction.0;
        self.column = self.column + self.direction.1;

        // out-of-bounds fix
        if self.row < 0 {
            self.row = dimensions.0 - 1;
        }
        if self.row >= dimensions.0 {
            self.row = 0;
        }
        if self.column < 0 {
            self.column = dimensions.1 - 1;
        }
    
        if self.column >= dimensions.1 {
            self.column = 0;
        }

        // add segments if they are missing
        if self.body.len() > 0 {
            self.body[0].row = self.row;
            self.body[0].column = self.column;
        } else {
            self.init();
        }
        
        for x in (1..self.len).rev() {
            let segment = SnakeSegment {
                row: self.body[(x - 1) as usize].row,
                column: self.body[(x - 1) as usize].column,
                color: Cell {red: 128_u8, green: 0_u8, blue: 0_u8},
            };
            if self.body.len() as i32 == self.len {
                self.body[x as usize] = segment;
            } else {
                self.body.push(segment);
            }
        }
    }

    pub fn init(&mut self) {
        // init works ok
        self.body = Vec::new();
        for x in 0..self.len {
            let segment = SnakeSegment {
                row: self.row,
                column: self.column + x,
                color: Cell {red: 128_u8, green: 0_u8, blue: 0_u8},
            };
            self.body.push(segment);
        }
    }

    // self-collision
    // not very fluid though
    pub fn collides(&mut self) -> bool {
        for index in  2..self.body.len() {
            if self.row == self.body[index].row && self.column == self.body[index].column {
                println!("{:?} {:?} vs {:?} {:?} on {:?}",
                    self.row, self.column,
                    self.body[index].row, self.body[index].column,
                    index
                );
                return true
            }
        }
        false
    }

    // head vs food collision
    pub fn collides_food(&mut self, food: (i32, i32)) -> bool {
        self.row == food.0 && self.column == food.1
    }

    pub fn increment(&mut self) {
        self.len += 1;
    }
}
