use std::io;
use std::io::Write;

struct Board {
    b: [[i32; 8]; 8]
}
/*
Board construction:
        a        b         c         d       e         f         g       h
1    W_tower, W_knight, W_bishop, W_king, W_queen, W_bishop, W_knight, W_tower
2    W_pawn,  W_pawn,   W_pawn,   W_pawn, W_pawn,  W_pawn,   W_pawn,   W_pawn
3    ""       ""        ""        ""      ""       ""        ""        ""
4    ""       ""        ""        ""      ""       ""        ""        ""
5    ""       ""        ""        ""      ""       ""        ""        ""
6    ""       ""        ""        ""      ""       ""        ""        ""
7    B_pawn,  B_pawn,   B_pawn,   B_pawn, B_pawn,  B_pawn,   B_pawn,   B_pawn
8    B_tower, B_knight, B_bishop, B_queen, B_king, B_bishop, B_knight, B_tower

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

Or Number representation:
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

impl Board {
    fn construct(&mut self) { //Fill the board with pieces
        // put the pawns on the board
        for i in 0..8 {
            self.b[1][i] = 1;
            self.b[6][i] = -1;
        }

        // put non pawns on the board
        let mut start = 2;
        for i in 0..8 {
            self.b[0][i] = start;
            self.b[7][i] = start * -1;
            start += 1;
            if start == 6 {
                start -= 2;
            }
        }
        // King and queen should be on opposite sides
        self.b[7][3] = -6;
        self.b[7][4] = -5;
        
        }
}

fn main() {

    // Create a board 
    let mut board = Board{
        b: [[0; 8]; 8]
    }; 

    // Construct the board
    board.construct();

    // print the board
    for i in 0..8 {
        for j in 0..8 {
            if board.b[i][j] >= 0 {
                print!("  {}", board.b[i][j]);

            } else {
                print!(" {}", board.b[i][j]);
            }
        }
        io::stdout().flush().unwrap();
        println!("");
    }
}
