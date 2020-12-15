mod pieces;
mod tests;
mod board;
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
        b: [[0; 8]; 8]
    }; 

    // Construct the board
    board.construct();

    // print the starting board
    board.print_b();

    loop {
        board.user_move();
        board.print_b();
    }

    // let king: pieces::King = pieces::King{pos: [5, 1], key: 5};
    // let out: Vec<[i8; 2]> = king.move_set(&board);
    // for i in 0..out.len() {
    //     println!("{:?}", out[i]);
    // }
    // println!("");

    // let valid: Vec<[i8; 2]> = board.bound_check(out);
    // for i in 0..valid.len() {
    //     println!("{:?}", valid[i]);
    // }

    // let w: Box<dyn Moves> = pieces::piece_type(6, [0, 10]);
    // println!("{:?}", w.get_pos());
}
