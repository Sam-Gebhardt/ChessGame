use std::io;
use std::io::Write;
use crate::pieces::piece_type;

fn alpha(n: i8) -> &'static str {
    // Converts index to alpha char
    let a: &'static str = match n {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => "a",
    };
    return a;
}

pub struct Board {
    pub b: [[i8; 8]; 8]
}

impl Board {
    //Fill the board with pieces
    pub fn construct(&mut self) { 

        // put the pawns on the board
        for i in 0..8 {
            self.b[1][i] = 1;
            self.b[6][i] = -1;
        }

        // put non pawns on the board
        let mut start = 2;
        let mut change = 1;
        for i in 0..8 {
            self.b[0][i] = start;
            self.b[7][i] = start * -1;
            start += change;
            if start == 7 {
                start -= 3;
                change = -1;
            }
        }
        // King and queen should be on opposite sides
        self.b[7][3] = -6;
        self.b[7][4] = -5;
    }

    pub fn print_board(&self) {
        // print the board
        for i in 0..8 {
            for j in 0..8 {
                
                if j == 0 {}
    
                if self.b[i][j] >= 0 {
                    print!("  {}", self.b[i][j]);
    
                } else {
                    print!(" {}", self.b[i][j]);
                }
            }
            io::stdout().flush().unwrap();
            println!("");
        }
    }

    pub fn get_piece(&self, one: i8, two: i8) -> i8 {
        // Get a piece from the board at b[one][two]

        // Cast to correct type for indexing
        let one_usize: usize = one as usize;
        let two_usize: usize = two as usize;

        return self.b[one_usize][two_usize];
    }

    pub fn move_piece(&mut self, src: [i8; 2], dest: [i8; 2]) {
        // Move a piece from src to dest, set src to 0

        self.b[dest[0] as usize][dest[1] as usize] = self.get_piece(src[0], src[1]);
        self.b[src[0] as usize][src[1] as usize] = 0;
    }

    fn in_check(&self, src: [i8; 2], dest: [i8; 2]) -> bool {
        // See if a move cause a check to happen

        for i in 0..8 {
            for j in 0..8 {
                let piece = piece_type(self.get_piece(i, j), [i, j]);
            }
        }
        return true;
    }
}
