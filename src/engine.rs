use fxhash::FxHashMap;

use crate::board::{Board, Dir};

const RED_ZONE: usize = 100 * 1024; // 100k
const STACK_PER_RECURSION: usize = 1 * 1024 * 1024; // 1MB

pub(crate) struct Engine<const HEIGHT: usize, const WIDTH: usize> {
    start_board: Board<HEIGHT, WIDTH>,
    target_board: Board<HEIGHT, WIDTH>,
    visited: FxHashMap<Board<HEIGHT, WIDTH>, usize>,
    best: Vec<Dir>,
}

pub fn ensure_sufficient_stack<R, F: FnOnce() -> R>(f: F) -> R {
    stacker::maybe_grow(RED_ZONE, STACK_PER_RECURSION, f)
}

impl<const HEIGHT: usize, const WIDTH: usize> Engine<HEIGHT, WIDTH> {
    pub(crate) fn new(
        start_board: Board<HEIGHT, WIDTH>,
        target_board: Board<HEIGHT, WIDTH>,
    ) -> Self {
        Engine {
            start_board,
            target_board,
            visited: FxHashMap::default(),
            best: vec![],
        }
    }

    #[inline(never)]
    fn rec(&mut self, board: &mut Board<HEIGHT, WIDTH>, cur: &mut Vec<Dir>) {
        if let Some(l) = self.visited.get(board) {
            if cur.len() > *l {
                return;
            }
        }

        self.visited.insert(*board, cur.len());

        if board == &self.target_board {
            if cur.len() < self.best.len() || self.best.is_empty() {
                self.best = cur.clone();
            }

            return;
        }

        if board.can_move_left() {
            cur.push(Dir::Left);
            board.move_left();
            ensure_sufficient_stack(|| self.rec(board, cur));
            board.move_right();
            cur.pop();
        }

        if board.can_move_right() {
            cur.push(Dir::Right);
            board.move_right();
            ensure_sufficient_stack(|| self.rec(board, cur));
            board.move_left();
            cur.pop();
        }

        if board.can_move_up() {
            cur.push(Dir::Up);
            board.move_up();
            ensure_sufficient_stack(|| self.rec(board, cur));
            board.move_down();
            cur.pop();
        }

        if board.can_move_down() {
            cur.push(Dir::Down);
            board.move_down();
            ensure_sufficient_stack(|| self.rec(board, cur));
            board.move_up();
            cur.pop();
        }
    }

    pub(crate) fn solve(&mut self) -> Vec<Dir> {
        self.rec(&mut self.start_board.clone(), &mut vec![]);

        self.best.clone()
    }
}
