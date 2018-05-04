use Grid;

pub struct Game {
    pub grid: Grid,
}

impl Game {
    pub fn new() -> Game {
        Game { grid: Grid::new() }
    }
}
