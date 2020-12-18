/*
An opponent that move's are randomly selected
*/
use crate::pieces::Moves;
use crate::pieces::piece_type;
use crate::board::Board;
extern crate rand;
use rand::prelude::*;

pub fn random_move(board: &mut Board) {
    make_move(board);
}

fn make_move(board: &mut Board) {
    let _move: [[i8; 2]; 2] = choose_move(board);
    board.move_piece(_move[0], _move[1]);
}

fn choose_move(board: &mut Board) -> [[i8; 2]; 2] {
    // Opponent will always be black pieces

    let mut piece_list: Vec<Box<dyn Moves>> = Vec::new();
    let mut piece: Box<dyn Moves>;
    let mut moves: Vec<[i8; 2]>;

    for i in 0..8 {
        for j in 0..8 {
            if board.get_piece(i, j) < 0 {
                piece = piece_type(board.get_piece(i, j), [i, j]);
                moves = piece.move_set(&board);

                if moves.len() != 0 { //todo: Move out of loop
                    piece_list.push(piece);
                }
            }
        }
    }

    let select = thread_rng().gen_range(0, piece_list.len());
    moves = piece_list[select].move_set(&board);
    let r = thread_rng().gen_range(0, moves.len());
    
    return [piece_list[select].get_pos(), moves[r]];
}