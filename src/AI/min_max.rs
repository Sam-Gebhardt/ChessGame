/*
Chess AI that is built upon min/max with alpha pruning 

https://www.chessprogramming.org/Minimax
https://www.chessprogramming.org/Alpha-Beta
https://vitcapstoneproject.wordpress.com/2018/02/26/evaluating-a-board-position/

*/

use crate::board::Board;
use crate::pieces::piece_type;
use crate::pieces::sign_checker;
use crate::pieces::Moves;
use crate::AI::eval;
use crate::AI::random::print_move;


// takes a move_set vec and adds the src to each move 
// in order to allow min/max to work
fn add_source(src: [i8; 2], move_set: Vec<[i8; 2]>) -> Vec<[[i8; 2]; 2]> {

    let mut fixed: Vec<[[i8; 2]; 2]> = Vec::new();

    for i in 0..move_set.len() {
        fixed.push([src, move_set[i]]);
    }

    return fixed;
}


// Generate all the moves for the pieces speciefed by color
fn generate_all_moves(board: &Board, color: i8) -> Vec<[[i8; 2]; 2]> {

    let mut piece: Box<dyn Moves>;
    let mut moves: Vec<[[i8; 2]; 2]> = Vec::new();
    let mut key: i8;

    for i in 0..8 {
        for j in 0..8 {

            key = board.get_piece(i, j);
            if sign_checker(key, color) { 

                piece = piece_type(key, [i, j]);
                moves.append(&mut add_source([i, j], piece.move_set(board)));
            }
        }
    }
    return moves;
}


fn max(board: Board, mut best: &mut [[i8; 2]; 2], mut alpha: i32, beta: i32, depth: i32) -> i32 {

    if depth == 0 {
        return eval::eval_board(&board, 1);
    }

    let mut score: i32;
    let mut board_copy: Board;
    let moves: Vec<[[i8; 2]; 2]> = generate_all_moves(&board, -1);

    for i in 0..moves.len() {

        board_copy = board.clone();
        board_copy.move_piece(moves[i][0], moves[i][1]);
        score = min(board_copy, &mut best, alpha, beta, depth - 1);

        if score >= beta {
            return beta;
        } if score > alpha {
            *best = moves[i];
            alpha = score;
        }
    }
    return alpha;
}


fn min(board: Board, mut best: &mut [[i8; 2]; 2], alpha: i32, mut beta: i32, depth: i32) -> i32 {

    if depth == 0 {
        return eval::eval_board(&board, -1); 
    }

    let mut score: i32;
    let mut board_copy: Board;
    let moves: Vec<[[i8; 2]; 2]> = generate_all_moves(&board, 1);

    for i in 0..moves.len() {

        board_copy = board.clone();
        board_copy.move_piece(moves[i][0], moves[i][1]);
        score = max(board_copy, &mut best, alpha, beta, depth - 1);

        if score <= alpha {
            return alpha;
        } if score < beta {
            beta = score;
        }
    }

    return beta;
}


pub fn select(board: &mut Board) {

    let board_copy: Board = board.clone();

    // the best move will be the first element in the vector
    let mut best_move: [[i8; 2]; 2] = [[0, 0], [0, 0]];

    // Run with a depth of 3 as default
    let _score: i32 = max(board_copy, &mut best_move, -9999999, 9999999, 3);
     
    board.move_piece(best_move[0], best_move[1]);
    print_move(best_move);
    // return best_move[0];
}

/*
Todo:
Copying the board each time has a lot of over head, 
tmp move piece each time then reset?

Moves are being made that from future board:
    *Need to save the first move that leads to the best board

Can make general improvements:
    * See if the move puts the player in check
*/