mod pieces;
mod board;
mod test_board;
mod test_pieces;
#[allow(unused_imports)]
use AI::random::random_move;
use AI::min_max::select;
#[allow(non_snake_case)]
#[allow(unused_imports)]
#[allow(dead_code)]
mod AI { pub mod random; pub mod min_max; pub mod eval; pub mod test_AI; }


/*
Board construction:
      a      b       c       d       e       f       g       h
----------------------------------------------------------------
1|   -2     -3      -4      -6      -5      -4      -3      -2
2|   -1     -1      -1      -1      -1      -1      -1      -1
3|    0      0       0       0       0       0       0       0
4|    0      0       0       0       0       0       0       0
5|    0      0       0       0       0       0       0       0
6|    0      0       0       0       0       0       0       0
7|    1      1       1       1       1       1       1       1 
8|    2      3       4       5       6       4       3       2

w = positive
b = negative
empty = 0
pawn = 1
tower = 2
knight = 3
bishop = 4
king = 5
queen = 6
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

    loop {
        if !(board.user_move()) {
            break;
        }

        if board.check_mate(-1) {
            println!("Game over! \nWhite wins");
            board.print_b();
            break;
        } else if board.stalemate(-1) {
            println!("Stalemate!");
            break;
        }

        // An opponent that chooses a random piece to move
        // random_move(&mut board);

        // Move for min/max algo
        select(&mut board);

        if board.check_mate(1) {
            println!("Game over! \nBlack wins");
            board.print_b();
            break;
        } else if board.stalemate(1) {
            println!("Stalemate!");
            break;
        }

        board.print_b();
    }
}
