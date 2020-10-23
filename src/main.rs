#![feature(min_const_generics)]

mod board;
mod engine;

fn main() {
    println!("Solving 4x2...");

    let start_pos: [[usize; 4]; 2] = [[7, 4, 0, 3], [5, 2, 1, 6]];
    let target_pos: [[usize; 4]; 2] = [[4, 1, 2, 3], [7, 5, 6, 0]];

    let start_board = board::Board::<2, 4>::new(start_pos);
    let target_board = board::Board::<2, 4>::new(target_pos);

    let mut eng = engine::Engine::new(start_board, target_board);
    let res = eng.solve();

    println!("{:?}", res);

    // example puzzle from https://www.youtube.com/watch?v=NoUFgLGVVgw

    println!("Solving 3x3...");

    let start_pos: [[usize; 3]; 3] = [[2, 3, 6], [5, 7, 4], [0, 1, 8]];
    let target_pos: [[usize; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 0]];

    let start_board = board::Board::<3, 3>::new(start_pos);
    let target_board = board::Board::<3, 3>::new(target_pos);

    let mut eng = engine::Engine::new(start_board, target_board);
    let res = eng.solve();

    println!("{:?}", res);
}
