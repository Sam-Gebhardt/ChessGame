mod pieces;
mod board;
use AI::random::random_move;
mod AI { pub mod random; }


// use crate::pieces::Moves;
// use crate::board;

/*
Board construction:
      a      b       c       d       e       f       g       h
----------------------------------------------------------------
1|    2      3       4       5       6       4       3       2
2|    1      1       1       1       1       1       1       1 
3|    0      0       0       0       0       0       0       0
4|    0      0       0       0       0       0       0       0
5|    0      0       0       0       0       0       0       0
6|    0      0       0       0       0       0       0       0
7|    -1     -1      -1      -1      -1      -1      -1      -1
8|    -2     -3      -4      -6      -5      -4      -3      -2

w = positive
b = negative
empty = 0
pawn = 1
tower = 2
knight = 3
bishop = 4
king = 5
queen = 6

Each piece is a heap allocated trait 
*/


fn main() {

    // Create a board         
    let mut board = board::Board{
        b: [[0; 8]; 8],
        white: [0, 0],
        black: [0, 0]
    }; 

    // Construct the board
    board.construct();

    // print the starting board
    board.print_b();
    // for _i in 0..5 {
    //     random::choose_move(&mut board);
    // }
    loop {
        if !(board.user_move()) {
            break;
        }
        if board.check_mate() {
            println!("Game over!");
            break;
        }

        // An opponent that chooses a random piece to move
        random_move(&mut board);

        if board.check_mate() {
            println!("Game over!");
            break;
        }

        board.print_b();
    }
}
