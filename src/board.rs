use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashSet;
use std::convert::From;

const HOLE: usize = 0;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Hash, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
pub(crate) struct RowCol(usize, usize);

impl From<(usize, usize)> for RowCol {
    fn from(item: (usize, usize)) -> RowCol {
        RowCol(item.0, item.1)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub(crate) struct Board<const HEIGHT: usize, const WIDTH: usize> {
    board: [[usize; WIDTH]; HEIGHT],

    hole_pos: RowCol,
}

impl<const HEIGHT: usize, const WIDTH: usize> Board<HEIGHT, WIDTH> {
    pub(crate) fn new(position: [[usize; WIDTH]; HEIGHT]) -> Self {
        let mut tmp = Board {
            board: [[HOLE; WIDTH]; HEIGHT],
            hole_pos: RowCol(0, 0),
        };

        tmp.setup(position);
        tmp
    }

    pub(crate) fn setup(&mut self, position: [[usize; WIDTH]; HEIGHT]) -> &mut Self {
        let mut hole_found = false;
        let mut duplicates: HashSet<usize> = HashSet::new();

        for (row_idx, row) in position.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                if *tile == HOLE {
                    // only one empty cell is allowed
                    assert!(!hole_found);
                    hole_found = true;
                    self.hole_pos = RowCol(row_idx, col_idx)
                }

                // insure there are no duplicates on a board
                assert!(duplicates.get(&tile).is_none());
                duplicates.insert(*tile);

                self.board[row_idx][col_idx] = *tile;
            }
        }

        self
    }

    #[inline(always)]
    fn get(&self, row_col: RowCol) -> usize {
        assert!(row_col.0 < HEIGHT);
        assert!(row_col.1 < WIDTH);

        self.board[row_col.0][row_col.1]
    }

    #[inline(always)]
    fn set(&mut self, row_col: RowCol, val: usize) {
        assert!(row_col.0 < HEIGHT);
        assert!(row_col.1 < WIDTH);

        self.board[row_col.0][row_col.1] = val
    }

    #[inline(always)]
    fn swap_cells(&mut self, from: RowCol, to: RowCol) {
        self.set(from, self.get(to));
        self.set(to, HOLE);
        self.hole_pos = to;
    }

    #[inline(always)]
    pub(crate) fn move_hole(&mut self, dir: Dir) -> RowCol {
        match dir {
            Dir::Left => {
                self.swap_cells(self.hole_pos, RowCol(self.hole_pos.0, self.hole_pos.1 - 1))
            }
            Dir::Right => {
                self.swap_cells(self.hole_pos, RowCol(self.hole_pos.0, self.hole_pos.1 + 1))
            }
            Dir::Up => self.swap_cells(self.hole_pos, RowCol(self.hole_pos.0 - 1, self.hole_pos.1)),
            Dir::Down => {
                self.swap_cells(self.hole_pos, RowCol(self.hole_pos.0 + 1, self.hole_pos.1))
            }
        }
        RowCol(self.hole_pos.0, self.hole_pos.1)
    }

    #[inline(always)]
    pub(crate) fn can_move_left(&self) -> bool {
        self.hole_pos.1 > 0
    }

    #[inline(always)]
    pub(crate) fn move_left(&mut self) -> &mut Self {
        self.move_hole(Dir::Left);
        self
    }

    #[inline(always)]
    pub(crate) fn can_move_right(&self) -> bool {
        self.hole_pos.1 < WIDTH - 1
    }

    #[inline(always)]
    pub(crate) fn move_right(&mut self) -> &mut Self {
        self.move_hole(Dir::Right);
        self
    }

    #[inline(always)]
    pub(crate) fn can_move_up(&self) -> bool {
        self.hole_pos.0 > 0
    }

    #[inline(always)]
    pub(crate) fn move_up(&mut self) -> &mut Self {
        self.move_hole(Dir::Up);
        self
    }

    #[inline(always)]
    pub(crate) fn can_move_down(&self) -> bool {
        self.hole_pos.0 < HEIGHT - 1
    }

    #[inline(always)]
    pub(crate) fn move_down(&mut self) -> &mut Self {
        self.move_hole(Dir::Down);
        self
    }
}

impl<const HEIGHT: usize, const WIDTH: usize> PartialOrd for Board<HEIGHT, WIDTH> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for row_idx in 0..self.board.len() {
            for col_idx in 0..self.board[row_idx].len() {
                let tmp = self
                    .get(RowCol(row_idx, col_idx))
                    .cmp(&other.get(RowCol(row_idx, col_idx)));
                if tmp != Ordering::Equal {
                    return Some(tmp);
                }
            }
        }

        Some(Ordering::Equal)
    }
}
