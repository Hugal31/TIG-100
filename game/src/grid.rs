use {Cell, CodeCell};

#[derive(Clone, Debug)]
pub struct Grid {
    pub cells: [[Cell; Grid::HEIGHT]; Grid::WIDTH],
}

impl Grid {
    pub const HEIGHT: usize = 3;
    pub const WIDTH: usize = 4;
}

impl Default for Grid {
    fn default() -> Self {
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
