/*
Chess AI that is built upon min/max with alpha pruning 
https://www.chessprogramming.org/Minimax
https://www.chessprogramming.org/Alpha-Beta

https://vitcapstoneproject.wordpress.com/2018/02/26/evaluating-a-board-position/

*/

use crate::board::Board;
use crate::pieces::piece_type;
use crate::pieces::Moves;
use crate::AI::eval;


// takes a move_set vec and adds the src to each move 
// in order to allow min/max to work
fn add_source(src: [i8; 2], move_set: Vec<[i8; 2]>) -> Vec<[[i8; 2]; 2]> {

    let mut fixed: Vec<[[i8; 2]; 2]> = Vec::new();

    for i in 0..move_set.len() {
        fixed.push([src, move_set[i]]);
    }

    return fixed;
}


// Generate all the moves for the AI controlled pieces
fn generate_all_moves(board: &Board) -> Vec<[[i8; 2]; 2]> {

    let mut piece: Box<dyn Moves>;
    let mut moves: Vec<[[i8; 2]; 2]> = Vec::new();
    let mut key: i8;

    for i in 0..8 {
        for j in 0..8 {

            key = board.get_piece(i, j);
            if key < 0 { 
                piece = piece_type(key, [i, j]);
                moves.append(&mut add_source([i, j], piece.move_set(board)));
            }
        }
    }

    return moves;
}


fn max(board: &Board, mut alpha: i32, beta: i32, depth: i32) -> i32 {

    if depth == 0 {
        return eval::eval_board(board, -1);
    }

    let mut score: i32;
    let moves: Vec<[[i8; 2]; 2]> = generate_all_moves(board);

    for _i in 0..moves.len() {
        score = min(board, alpha, beta, depth - 1);

        if score >= beta {
            return beta;
        } else if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}


fn min(board: &Board, alpha: i32, mut beta: i32, depth: i32) -> i32 {

    if depth == 0 {
        return eval::eval_board(board, 1); 
    }

    let mut score: i32;
    let moves: Vec<[[i8; 2]; 2]> = generate_all_moves(board);

    for _i in 0..moves.len() {
        score = max(board, alpha, beta, depth - 1);

        if score <= alpha {
            return alpha;
        } else if score < beta {
            beta = score;
        }
    }
    return beta;
}


pub fn select(board: &Board) {
    // Driver code

    let board_copy: Board = board.clone();

    // Run with a depth of 5 as default
    let score: i32 = max(&board_copy, -99999, 99999, 5);
    
    if score > 1 {
        return;
    }
}