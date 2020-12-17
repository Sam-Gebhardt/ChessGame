/*
An opponent that move's are randomly selected
*/
use crate::pieces::Moves;
use crate::pieces::piece_type;
use crate::board::Board;
extern crate rand;
use rand::thread_rng;
use rand::Rng;

pub fn random_move() {

}

fn make_move() {

}

pub fn choose_move(board: &mut Board) -> [i8; 2] {
    // Opponent will always be black pieces

    let mut moves: Vec<[i8; 2]> = Vec::new();
    let mut piece: Box<dyn Moves>;

    for i in 0..8 {
        for j in 0..8 {
            if board.get_piece(i, j) < 0 {
                piece = piece_type(board.get_piece(i, j), [i, j]);
                let v: Vec<[i8; 2]> = piece.move_set(&board);
                for m in 0..v.len() {
                    moves.push(v[m]);
                }
            }
        }
    }
    let y: f64 = rng.gen_range(-10.0, 10.0);
    println!("{:?}", moves[y]);
    return moves[y];
}