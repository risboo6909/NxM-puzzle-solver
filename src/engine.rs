use std::cmp::min;

use fxhash::{hash, FxBuildHasher};
use lru::LruCache;

use crate::board::{Board, Dir};

const RED_ZONE: usize = 100 * 1024; // 100k
const STACK_PER_RECURSION: usize = 1 * 1024 * 1024; // 1MB

// the bigger this number the less optimal solution we get but at the same time
// search space reduces significantly
const RELAXATION_COEFF: isize = 7;

// evetually, maximum length of winning sequence of moves
const MAX_DEPTH: usize = 250;

const MAX_IMPROVE_ATTEMPTS: usize = 1 << 32;

const LRU_CACHE_ITEMS: usize = 1 << 26;

pub(crate) struct Engine<const HEIGHT: usize, const WIDTH: usize> {
    start_board: Board<HEIGHT, WIDTH>,
    target_board: Board<HEIGHT, WIDTH>,
    target_board_hash: usize,
    visited: LruCache<Board<HEIGHT, WIDTH>, u16, FxBuildHasher>,
    best: Vec<Dir>,
    moves_counter: usize,
    depth: usize,
    max_depth: usize,
    improve_attempts: usize,
    max_improve_attempts: usize,
    solution_found: bool,
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
            target_board_hash: hash(&target_board),
            visited: LruCache::with_hasher(LRU_CACHE_ITEMS, FxBuildHasher::default()),
            best: vec![],
            moves_counter: 0,
            depth: 0,
            max_depth: 0,
            improve_attempts: 0,
            max_improve_attempts: 0,
            solution_found: false,
        }
    }

    #[inline(never)]
    fn rec(&mut self, board: &mut Board<HEIGHT, WIDTH>, cur: &mut Vec<Dir>) {
        if self.solution_found && self.improve_attempts >= self.max_improve_attempts {
            return;
        }

        if self.depth > self.max_depth {
            self.max_depth = self.depth;
        }

        if self.depth >= MAX_DEPTH {
            return;
        }

        if let Some(l) = self.visited.get(board) {
            if *l as isize - cur.len() as isize <= RELAXATION_COEFF {
                return;
            }
        }

        self.visited.put(*board, cur.len() as u16);
        self.moves_counter += 1;
        self.improve_attempts += 1;

        if hash(board) == self.target_board_hash && board == &self.target_board {
            if cur.len() < self.best.len() || self.best.is_empty() {
                self.best = cur.clone();
                self.solution_found = true;
                self.improve_attempts = 0;
                self.max_improve_attempts = min(self.moves_counter << 4, MAX_IMPROVE_ATTEMPTS);
                println!(
                    "try to improve solution of length {} in {} moves",
                    self.best.len(),
                    self.max_improve_attempts
                );
            }

            return;
        }

        if board.can_move_left() {
            cur.push(Dir::Left);
            board.move_left();
            self.depth += 1;
            ensure_sufficient_stack(|| self.rec(board, cur));
            self.depth -= 1;
            board.move_right();
            cur.pop();
        }

        if self.depth == 0 {
            self.visited.clear();
        }

        if board.can_move_right() {
            cur.push(Dir::Right);
            board.move_right();
            self.depth += 1;
            ensure_sufficient_stack(|| self.rec(board, cur));
            self.depth -= 1;
            board.move_left();
            cur.pop();
        }

        if self.depth == 0 {
            self.visited.clear();
        }

        if board.can_move_up() {
            cur.push(Dir::Up);
            board.move_up();
            self.depth += 1;
            ensure_sufficient_stack(|| self.rec(board, cur));
            self.depth -= 1;
            board.move_down();
            cur.pop();
        }

        if self.depth == 0 {
            self.visited.clear();
        }

        if board.can_move_down() {
            cur.push(Dir::Down);
            board.move_down();
            self.depth += 1;
            ensure_sufficient_stack(|| self.rec(board, cur));
            self.depth -= 1;
            board.move_up();
            cur.pop();
        }
    }

    pub(crate) fn solve(&mut self) -> (Vec<Dir>, usize, usize) {
        self.moves_counter = 0;

        self.rec(&mut self.start_board.clone(), &mut vec![]);

        (self.best.clone(), self.moves_counter, self.max_depth)
    }
}
