/*
Chess AI that is built upon min/max with alpha pruning 
https://www.chessprogramming.org/Minimax
https://www.chessprogramming.org/Alpha-Beta

*/

use crate::board::Board;
use crate::pieces::piece_type;
use crate::pieces::Moves;


// Generate all the moves for the AI controlled pieces
fn generate_all_moves(board: &Board) -> Vec<[[i8; 2]; 2]>{

    let mut piece: Box<dyn Moves>;
    let mut moves: Vec<[[i8; 2]; 2]>;
    let mut key: i8;

    for i in 0..8 {
        for j in 0..8 {

            key = board.get_piece(i, j);
            if key < 0 {
                piece = piece_type(key, [i, j]);
                moves.push(piece.move_set(board));
            }
        }
    }

    return moves;
}


// Get a number value for the current state of the board
fn eval_board() {
    

}


fn max(alpha: i64, beta: i64, depth: i64) -> i64 {

    if depth == 0 {
        return 0; // An eval function that determines the value of the board
    }

    let mut score: i64;
    // let moves 

    for i in 0..moves.len() {
        score = min(alpha, beta, depth - 1);

        if score >= beta {
            return beat;
        } else if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}


fn min(alpha: i64, beta: i64, depth: i64) -> i64{

    if depth == 0 {
        return 0 * -1; // An eval function that determines the value of the board
    }

    let mut score: i64;

    for i in 0..moves.len() {
        score = max(alpha, beta, depth - 1);

        if score <= alpha {
            return alpha;
        } else if score < beta {
            beta = score;
        }
    }
    return beta;
}


pub fn select() {
    // Driver code
}