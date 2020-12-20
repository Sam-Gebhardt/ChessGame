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

    if board.in_check(board.check_mate_helper(), board.check_mate_helper(), -1) {
        let save = block(board);
        board.move_piece(save[0], save[1]);
    } else {
        let _move: [[i8; 2]; 2] = choose_move(board);
        board.move_piece(_move[0], _move[1]);
    }
}

fn block(board: &mut Board) -> [[i8; 2]; 2] {
    // Returns a move that protects the king

    let mut saving_moves: Vec<[[i8; 2]; 2]> = Vec::new();
    let mut moves: Vec<[i8; 2]>;

    for i in 0..8 {
        for j in 0..8 {

            if board.get_piece(i, j) < 0 {
                let piece: Box<dyn Moves> = piece_type(board.get_piece(i, j), [i, j]);
                moves = piece.move_set(board);

                for a in 0..moves.len() {
                    if !board.in_check(piece.get_pos(), moves[a], -1) {
                        saving_moves.push([piece.get_pos(), moves[a]]);
                    }
                }
            }
        }
    }
    if saving_moves.len() == 0 {
        return [[0, 0], [0, 0]]; // Indicates check mate
    }
    let select = thread_rng().gen_range(0, saving_moves.len());
    return saving_moves[select];
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