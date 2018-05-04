use {Cell, CodeCell};

#[derive(Clone, Debug)]
pub struct Grid {
    pub cells: [[Cell; Grid::WIDTH]; Grid::HEIGHT],
}

impl Grid {
    pub const HEIGHT: usize = 3;
    pub const WIDTH: usize = 4;
    
    pub fn new() -> Grid {
        Grid {
            cells: [
                [
                    CodeCell::default().into(),
                    CodeCell::default().into(),
                    CodeCell::default().into(),
                ],
                [
                    CodeCell::default().into(),
                    CodeCell::default().into(),
                    CodeCell::default().into(),
                ],
                [
                    CodeCell::default().into(),
                    CodeCell::default().into(),
                    CodeCell::default().into(),
                ],
                [
                    CodeCell::default().into(),
                    CodeCell::default().into(),
                    CodeCell::default().into(),
                ],
            ],
        }
    }
}
