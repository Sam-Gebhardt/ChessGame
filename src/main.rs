use std::io;
use std::io::Write;
mod pieces;

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
    let empty: pieces::Empty = pieces::Empty{pos: [0, 0], key: 0};
    let mut board = pieces::Board{
        b: [[Box::new(empty); 8]; 8]
    }; 

    // Construct the board
    board.construct();

    // print the board
    for i in 0..8 {
        for j in 0..8 {
            if board.b[i][j].get_key() >= 0 {
                print!("  {}", board.b[i][j].get_key());

            } else {
                print!(" {}", board.b[i][j].get_key());
            }
        }
        io::stdout().flush().unwrap();
        println!("");
    }

}
